extern crate libc;

use libc::c_void;

extern "C" {
    fn for_each_interesting_number(callback: extern "C" fn(i32, *mut c_void), data: *mut c_void);
}

extern "C" fn print_from_raw(num: i32, _: *mut c_void) {
    println!("num = {}", num);
}

fn pretest1_print_from_callback() {
    println!("\nbelow is pretest1_print_from_callback");

    unsafe {
        for_each_interesting_number(print_from_raw, std::ptr::null_mut());
    }
}

extern "C" fn accumulate_(num: i32, data: *mut c_void) {
    unsafe { *data.cast::<i32>() += num };
}

fn internal_accumulate(vec: &mut Vec<i32>) {
    unsafe {
        for_each_interesting_number(accumulate_, vec.as_mut_ptr() as *mut c_void);
    }
}

fn pretest2_accumulate_into_vec() {
    println!("\nbelow is pretest2_accumulate_into_vec");
    let mut vec = vec![0i32];
    internal_accumulate(&mut vec);
    println!("after accumulate, vec[0] value = {}", vec[0]);
}

extern "C" fn closure_internal<F: FnMut(i32)>(num: i32, closure: *mut c_void) {
    let closure: &mut F = unsafe { &mut *(closure as *mut F) };
    closure(num);
}

extern "C" fn for_each_number<F: FnMut(i32)>(mut closure: F) {
    let closure_ptr: *mut c_void = &mut closure as *mut F as *mut c_void;
    unsafe { for_each_interesting_number(closure_internal::<F>, closure_ptr) };
}

fn pretest3_callback_from_callback() {
    println!("\nbelow is pretest3_callback_from_callback");

    for_each_number(|num| {
        println!("Received number: {}", num);
    });
}

extern "C" fn accumulate_internal<F: FnMut(i32, *mut c_void)>(
    num: i32,
    closure_and_data: *mut c_void,
) {
    let combined: &mut (&mut F, *mut c_void) =
        unsafe { &mut *(closure_and_data as *mut (&mut F, *mut c_void)) };
    (combined.0)(num, combined.1);
}

extern "C" fn run_for_each_with_data<F: FnMut(i32, *mut c_void)>(
    mut closure: F,
    data: *mut c_void,
) {
    let mut combined = (&mut closure, data);
    let ptr: *mut c_void = &mut combined as *mut _ as *mut c_void;
    unsafe {
        for_each_interesting_number(accumulate_internal::<F>, ptr);
    }
}

fn pretest4_accumulate_from_callback() {
    println!("\nbelow is pretest4_accumulate_from_callback");

    let mut vec = vec![0i32];
    let vec_ptr = vec.as_mut_ptr() as *mut c_void;

    run_for_each_with_data(
        |num, vec_ptr: *mut c_void| {
            unsafe { *vec_ptr.cast::<i32>() += num };
        },
        vec_ptr,
    );

    println!("after accumulate, vec[0] value = {}", vec[0]);
}

fn main() {
    pretest1_print_from_callback();
    pretest2_accumulate_into_vec();
    pretest3_callback_from_callback();
    pretest4_accumulate_from_callback();
}
