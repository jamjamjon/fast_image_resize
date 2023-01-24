use crate::pixels::{U8x3, U8x4};
use std::arch::wasm32::*;
use std::intrinsics::transmute;

#[inline(always)]
pub unsafe fn load_v128<T>(buf: &[T], index: usize) -> v128 {
    v128_load(buf.get_unchecked(index..).as_ptr() as *const v128)
}

#[inline(always)]
pub unsafe fn loadl_i64<T>(buf: &[T], index: usize) -> v128 {
    let i = buf.get_unchecked(index..).as_ptr() as *const i64;
    i64x2(*i, 0)
}

#[inline(always)]
pub unsafe fn loadl_i32<T>(buf: &[T], index: usize) -> v128 {
    let i = buf.get_unchecked(index..).as_ptr() as *const i32;
    i32x4(*i, 0, 0, 0)
}

#[inline(always)]
pub unsafe fn loadl_i16<T>(buf: &[T], index: usize) -> v128 {
    let i = buf.get_unchecked(index..).as_ptr() as *const i16;
    i16x8(*i, 0, 0, 0, 0, 0, 0, 0)
}

#[inline(always)]
pub unsafe fn ptr_i16_to_set1_i64(buf: &[i16], index: usize) -> v128 {
    i64x2_splat(*(buf.get_unchecked(index..).as_ptr() as *const i64))
}

#[inline(always)]
pub unsafe fn ptr_i16_to_set1_i32(buf: &[i16], index: usize) -> v128 {
    i32x4_splat(*(buf.get_unchecked(index..).as_ptr() as *const i32))
}

#[inline(always)]
pub unsafe fn i32x4_extend_low_ptr_u8(buf: &[u8], index: usize) -> v128 {
    let ptr = buf.get_unchecked(index..).as_ptr() as *const v128;
    u32x4_extend_low_u16x8(i16x8_extend_low_u8x16(v128_load(ptr)))
}

#[inline(always)]
pub unsafe fn i32x4_extend_low_ptr_u8x4(buf: &[U8x4], index: usize) -> v128 {
    let v: u32 = transmute(buf.get_unchecked(index).0);
    u32x4_extend_low_u16x8(i16x8_extend_low_u8x16(u32x4(v, 0, 0, 0)))
}

#[inline(always)]
pub unsafe fn i32x4_extend_low_ptr_u8x3(buf: &[U8x3], index: usize) -> v128 {
    let pixel = buf.get_unchecked(index).0;
    i32x4(pixel[0] as i32, pixel[1] as i32, pixel[2] as i32, 0)
}

#[inline(always)]
pub unsafe fn i32x4_v128_from_u8(buf: &[u8], index: usize) -> v128 {
    let ptr = buf.get_unchecked(index..).as_ptr() as *const i32;
    i32x4(*ptr, 0, 0, 0)
}

#[inline(always)]
pub unsafe fn u16x8_mul_hi(a: v128, b: v128) -> v128 {
    let lo = u32x4_extmul_low_u16x8(a, b);
    let hi = u32x4_extmul_high_u16x8(a, b);
    i16x8_shuffle::<1, 3, 5, 7, 9, 11, 13, 15>(lo, hi)
}

#[inline(always)]
pub unsafe fn i64x2_mul_lo(a: v128, b: v128) -> v128 {
    const SHUFFLE: v128 = i8x16(0, 1, 2, 3, 8, 9, 10, 11, -1, -1, -1, -1, -1, -1, -1, -1);
    i64x2_extmul_low_i32x4(i8x16_swizzle(a, SHUFFLE), i8x16_swizzle(b, SHUFFLE))
}
