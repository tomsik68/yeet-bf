use bf::BfInst::{self, *};

use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use llvm_compiler::jump_table::compute_jump_table;
use std::collections::HashMap;

type BrainfProgramFunc = unsafe extern "C" fn();

pub fn compile_brainf(prog: Vec<BfInst>) -> Option<Module> {
    let jump_table = compute_jump_table(&prog);

    let ctx = Context::get_global();
    let module = ctx.create_module("brainfuck");
    let builder = ctx.create_builder();
    let fn_type = ctx.void_type();
    let putchar = module.add_function(
        "putchar",
        ctx.void_type()
            .fn_type(&[ctx.i8_type().as_basic_type_enum()], false),
        None,
    );

    let getchar = module.add_function("getchar", ctx.i8_type().fn_type(&[], false), None);

    let function = module.add_function("main", fn_type.fn_type(&[], false), None);

    let block = function.append_basic_block("entry");
    builder.position_at_end(&block);

    // TODO: decide if memory is large enough to be allocated on heap
    let ptr = builder.build_alloca(ctx.i64_type().ptr_type(AddressSpace::Generic), "ptr");

    let mem_size = ctx.i64_type().const_int(65536, false);
    let mem = builder.build_array_malloc(ctx.i64_type(), mem_size.clone(), "memory");

    let zero = ctx.i64_type().const_int(0, false);
    let one = ctx.i64_type().const_int(1, false);
    let minus_one = ctx.i64_type().const_int((-1i64) as u64, false);

    unsafe {
        let mem_begin = builder.build_gep(mem, &[zero], "mem_begin");
        builder.build_store(ptr, mem_begin);
    }

    let mut end_blocks = HashMap::new();

    for (ref inst, pc) in prog.iter().zip(0..) {
        match inst {
            LoopStart => {
                let loop_start = function.append_basic_block("");
                // add jump from previous block
                builder.build_unconditional_branch(&loop_start);
                // compare instructions should be placed into this block
                builder.position_at_end(&loop_start);

                // compare the val with zero
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let val = builder.build_load(ptr_val, "").as_int_value().clone();
                let result = builder.build_int_compare(IntPredicate::NE, val, zero, "");

                let loop_body = function.append_basic_block("");

                let loop_end = function.append_basic_block("");

                builder.build_conditional_branch(result, &loop_body, &loop_end);

                builder.position_at_end(&loop_body);

                end_blocks.insert(pc, (loop_body, loop_end));
            }

            LoopEnd => {
                let start_addr = jump_table.get(&pc).unwrap();
                let (loop_body, loop_end) = end_blocks
                    .get(start_addr)
                    .expect("compile_brainf: no end block found");

                // builder is at loop body right now
                builder.build_unconditional_branch(&loop_end);
                builder.position_at_end(&loop_end);

                // compare the val with zero
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let val = builder.build_load(ptr_val, "").as_int_value().clone();
                let result = builder.build_int_compare(IntPredicate::EQ, val, zero, "");

                let continuation = function.append_basic_block("");
                builder.build_conditional_branch(result, &continuation, &loop_body);
                builder.position_at_end(&continuation);
            }
            Inc => {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let val = builder.build_load(ptr_val, "");
                let new_val = builder.build_int_add(val.as_int_value().clone(), one.clone(), "");
                builder.build_store(ptr_val, new_val);
            }
            Dec => {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let val = builder.build_load(ptr_val, "");
                let new_val = builder.build_int_sub(val.as_int_value().clone(), one.clone(), "");
                builder.build_store(ptr_val, new_val);
            }
            IncPtr => unsafe {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let new_ptr = builder.build_gep(ptr_val, &[one], "");
                builder.build_store(ptr.clone(), new_ptr);
            },
            DecPtr => unsafe {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let new_ptr = builder.build_gep(ptr_val, &[minus_one], "");
                builder.build_store(ptr.clone(), new_ptr);
            },
            Read => {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let new_char = builder.build_call(getchar.clone(), &[], "");
                let val = builder.build_int_z_extend(
                    new_char
                        .try_as_basic_value()
                        .left()
                        .expect("getchar must return non-void")
                        .as_int_value()
                        .clone(),
                    ctx.i64_type(),
                    "",
                );
                builder.build_store(ptr_val, val);
            }
            Write => {
                let ptr_val = builder.build_load(ptr, "").as_pointer_value().clone();
                let val = builder.build_load(ptr_val, "");
                let c = builder.build_int_truncate(val.as_int_value().clone(), ctx.i8_type(), "");
                builder.build_call(putchar.clone(), &[c.as_basic_value_enum()], "");
            }
        }
    }

    builder.build_return(None);
    // TODO run LLVM verifier pass

    Some(module)
}
