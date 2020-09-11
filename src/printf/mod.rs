
mod format;
use core::ffi::c_void;
use core::slice::{
    from_raw_parts,
    from_raw_parts_mut,
};
use crate::printf::format::FormatString;
use crate::variadic::{va_list, VaList};
use crate::strlen::strlen;

extern "C" {
    pub fn snprintf(
        str: *mut u8,
        size: usize,
        format: *const u8,
        ...
    ) -> i32;
}

#[no_mangle]
pub extern "C" fn vsnprintf(
    str: *mut u8,
    size: usize,
    format: *const u8,
    ap: va_list,
) -> i32 {
    let mut va_list = VaList::from(ap);
    let format_len = strlen(format);
    let format = unsafe { from_raw_parts( format, format_len)};
    let mut output = unsafe { from_raw_parts_mut(str, size) };

    vnsprintf_rs( output, format, &mut va_list ) as i32
}

pub fn vnsprintf_rs(output: &mut [u8], format: &[u8], va_list: &mut VaList) -> usize {
    let format = FormatString::from(format);
    let mut formatted = format.merge(output, va_list);
    formatted.len()
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;

    #[test]
    fn vnsprintf() {
        let mut output: [u8; 128] = [0; 128];

        super::vnsprintf( &mut output, "Hi there %d", &[
            &42 as *const _ as *const c_void
        ]);

    }
}