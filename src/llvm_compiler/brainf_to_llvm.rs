use bf::BfInst::{self, *};

use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::BasicValue;
use inkwell::AddressSpace;

type BrainfProgramFunc = unsafe extern "C" fn();

pub fn compile_brainf(prog: Vec<BfInst>) -> Option<Module> {
    let ctx = Context::get_global();
    let module = ctx.create_module("brainfuck");
    let builder = ctx.create_builder();
    let fn_type = ctx.void_type();
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

    for inst in &prog {
        match inst {
            LoopStart => {
                // TODO
            }
            LoopEnd => {
                // TODO
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
            // TODO: solve read & write with syscalls
            Read => {}
            Write => {}
        }
    }

    builder.build_return(None);
    // TODO run LLVM verifier pass

    Some(module)
}
