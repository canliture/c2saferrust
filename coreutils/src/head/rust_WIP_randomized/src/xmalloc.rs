












use std::option::Option;

use std::slice;

use std::string::String;

use std::convert::TryInto;

use std::vec::Vec;

use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut libc::c_void;
    fn xalloc_die();
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const DEFAULT_MXFAST_0: C2RustUnnamed_0 = 128;
pub type C2RustUnnamed_0 = libc::c_uint;
#[inline]
fn irealloc(p: *mut libc::c_void, s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let new_size = if s == 0 { 1 } else { s };
        let new_ptr = unsafe { realloc(p, new_size as libc::c_ulong) };
        return new_ptr;
    } else {
        unsafe { return _gl_alloc_nomem(); }
    }
}

#[inline]
fn icalloc(n: i64, s: i64) -> Option<*mut libc::c_void> {
    let n_usize: usize = n.try_into().ok()?;
    let s_usize: usize = s.try_into().ok()?;

    if n_usize > usize::MAX / s_usize {
        if s_usize != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(std::ptr::null_mut()); // Allocating zero size
    }
    if s_usize > usize::MAX / n_usize {
        if n_usize != 0 {
            return None; // Equivalent to _gl_alloc_nomem()
        }
        return Some(std::ptr::null_mut()); // Allocating zero size
    }
    let total_size = n_usize.checked_mul(s_usize)?;
    let mut vec = vec![0u8; total_size]; // Allocating and zero-initializing the memory
    Some(vec.as_mut_ptr() as *mut libc::c_void) // Return as *mut libc::c_void
}

#[inline]
fn ireallocarray(
    mut p: Option<Vec<u8>>,
    mut n: usize,
    mut s: usize,
) -> Option<Vec<u8>> {
    if n <= usize::MAX && s <= usize::MAX {
        let mut nx: usize = n;
        let mut sx: usize = s;
        if n == 0 || s == 0 {
            sx = 1;
            nx = sx;
        }
        p = match p {
            Some(mut vec) => {
                vec.resize(nx * sx, 0);
                Some(vec)
            },
            None => Some(vec![0; nx * sx]),
        };
        return p;
    } else {
        _gl_alloc_nomem();
        return None;
    }
}

#[cold]
#[inline]
fn _gl_alloc_nomem() -> *mut libc::c_void {
    unsafe {
        *__errno_location() = 12 as libc::c_int;
    }
    return std::ptr::null_mut();
}

#[inline]
fn imalloc(s: usize) -> *mut libc::c_void {
    if s <= usize::MAX {
        let layout = std::alloc::Layout::from_size_align(s, 1).unwrap();
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            std::ptr::null_mut()
        } else {
            ptr as *mut libc::c_void
        }
    } else {
        std::ptr::null_mut()
    }
}

fn check_nonnull(p: *mut libc::c_void) -> *mut libc::c_void {
    if p.is_null() {
        unsafe { xalloc_die() };
    }
    p
}

#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut s: size_t) -> *mut libc::c_void {
    let ptr = malloc(s);
return check_nonnull(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn ximalloc(mut s: idx_t) -> *mut libc::c_void {
    let allocated_memory = imalloc(s.try_into().unwrap());
    return check_nonnull(allocated_memory);
}
#[no_mangle]
pub fn xcharalloc(n: usize) -> Vec<libc::c_char> {
    let layout = std::alloc::Layout::array::<libc::c_char>(n).unwrap();
    let ptr = unsafe {
        if std::mem::size_of::<libc::c_char>() == 1 {
            xmalloc(n.try_into().unwrap())
        } else {
    let n_usize: usize = n.try_into().unwrap();
    let size_of_char: usize = std::mem::size_of::<libc::c_char>();
    xnmalloc(n_usize, size_of_char)
}
    };
    
    let vec = unsafe { Vec::from_raw_parts(ptr as *mut libc::c_char, n, n) };
    vec
}

#[no_mangle]
pub unsafe extern "C" fn xrealloc(
    mut p: *mut libc::c_void,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = realloc(p, s);
    if r.is_null() && (p.is_null() || s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub fn xirealloc(p: Option<&mut [u8]>, s: usize) -> Option<Vec<u8>> {
    match p {
        Some(slice) => {
            let mut vec = Vec::with_capacity(s);
            vec.extend_from_slice(slice);
            Some(vec)
        },
        None => {
            let mut vec = Vec::with_capacity(s);
            Some(vec)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn xreallocarray(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = reallocarray(p, n, s);
    if r.is_null() && (p.is_null() || n != 0 && s != 0) {
        xalloc_die();
    }
    return r;
}
#[no_mangle]
pub fn xireallocarray(
    p: &mut Vec<u8>,
    n: usize,
    s: usize,
) -> Option<&mut [u8]> {
    let new_size = n.checked_mul(s)?;
    p.resize(new_size, 0);
    Some(&mut p[..new_size])
}

#[no_mangle]
pub fn xnmalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation");
    let mut vec = Vec::with_capacity(total_size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Vec from deallocating the memory
    ptr
}

#[no_mangle]
pub fn xinmalloc(n: usize, s: usize) -> Option<Vec<u8>> {
    let total_size = n.checked_mul(s)?;
    let result = vec![0u8; total_size];
    Some(result)
}

#[no_mangle]
pub unsafe extern "C" fn x2realloc(
    mut p: *mut libc::c_void,
    mut ps: *mut size_t,
) -> *mut libc::c_void {
    return x2nrealloc(p, ps, 1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
    } else {
        let (fresh0, fresh1) = n
            .overflowing_add(
                (n >> 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        *(&mut n as *mut size_t) = fresh0;
        if fresh1 {
            xalloc_die();
        }
    }
    p = xreallocarray(p, n, s);
    *pn = n;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn xpalloc(
    mut pa: *mut libc::c_void,
    mut pn: *mut idx_t,
    mut n_incr_min: idx_t,
    mut n_max: ptrdiff_t,
    mut s: idx_t,
) -> *mut libc::c_void {
    let mut n0: idx_t = *pn;
    let mut n: idx_t = 0;
    let (fresh2, fresh3) = n0.overflowing_add(n0 >> 1 as libc::c_int);
    *(&mut n as *mut idx_t) = fresh2;
    if fresh3 {
        n = 9223372036854775807 as libc::c_long;
    }
    if 0 as libc::c_int as libc::c_long <= n_max && n_max < n {
        n = n_max;
    }
    let mut nbytes: idx_t = 0;
    let mut adjusted_nbytes: idx_t = (if if (0 as libc::c_int as idx_t)
        < -(1 as libc::c_int) as idx_t
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && ((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { s })
            - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
        && (if s < 0 as libc::c_int as libc::c_long {
            if n < 0 as libc::c_int as libc::c_long {
                if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        -(1 as libc::c_int) as idx_t
                    }) + s
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                } else {
                    ((if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (s
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                    }) != 0
                    {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + -(1 as libc::c_int) as idx_t
                            >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        -(1 as libc::c_int) as idx_t / -s
                    }) <= -(1 as libc::c_int) as libc::c_long - n) as libc::c_int
                }
            } else {
                if (if (if ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                {
                    !(((((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 1 as libc::c_int as libc::c_long)
                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long)
                } else {
                    (if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t
                    }) + 0 as libc::c_int as libc::c_long
                }) < 0 as libc::c_int as libc::c_long
                {
                    (((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) + 0 as libc::c_int as idx_t)
                        < -(if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s
                            }) + 0 as libc::c_int as idx_t
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long
                        })) as libc::c_int
                } else {
                    ((0 as libc::c_int as libc::c_long)
                        < (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                {
                    if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        ((0 as libc::c_int as libc::c_long)
                            < n + 0 as libc::c_int as idx_t) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long) < n
                            && (-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                    }
                } else {
                    (0 as libc::c_int as idx_t / s < n) as libc::c_int
                }
            }
        } else {
            if s == 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else {
                if n < 0 as libc::c_int as libc::c_long {
                    if (if (if ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    {
                        !(((((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 1 as libc::c_int as libc::c_long)
                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long)
                    } else {
                        (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t
                        }) + 0 as libc::c_int as libc::c_long
                    }) < 0 as libc::c_int as libc::c_long
                    {
                        (((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            n
                        }) + 0 as libc::c_int as idx_t)
                            < -(if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long
                            })) as libc::c_int
                    } else {
                        ((0 as libc::c_int as libc::c_long)
                            < (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                n
                            }) + 0 as libc::c_int as idx_t) as libc::c_int
                    }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                    {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            s
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            ((0 as libc::c_int as libc::c_long)
                                < s + 0 as libc::c_int as idx_t) as libc::c_int
                        } else {
                            ((-(1 as libc::c_int) as libc::c_long
                                - 0 as libc::c_int as idx_t)
                                < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                        }
                    } else {
                        (0 as libc::c_int as idx_t / n < s) as libc::c_int
                    }
                } else {
                    (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                }
            }
        }) != 0
    {
        let (fresh8, _fresh9) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh8;
        1 as libc::c_int
    } else {
        let (fresh10, fresh11) = n.overflowing_mul(s);
        *(&mut nbytes as *mut idx_t) = fresh10;
        fresh11 as libc::c_int
    } != 0
    {
        if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        }
    } else {
        (if nbytes < DEFAULT_MXFAST_0 as libc::c_int as libc::c_long {
            DEFAULT_MXFAST_0 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_ulong
    }) as idx_t;
    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
        nbytes = adjusted_nbytes - adjusted_nbytes % s;
    }
    if pa.is_null() {
        *pn = 0 as libc::c_int as idx_t;
    }
    if n - n0 < n_incr_min
        && {
            let (fresh12, fresh13) = n0.overflowing_add(n_incr_min);
            *(&mut n as *mut idx_t) = fresh12;
            fresh13 as libc::c_int != 0
                || 0 as libc::c_int as libc::c_long <= n_max && n_max < n
                || (if (0 as libc::c_int as idx_t) < -(1 as libc::c_int) as idx_t
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        n
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        s
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && (if s < 0 as libc::c_int as libc::c_long {
                        if n < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    -(1 as libc::c_int) as idx_t
                                }) + s
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (n < -(1 as libc::c_int) as idx_t / s) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (s
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < s) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + -(1 as libc::c_int) as idx_t
                                        >> (::core::mem::size_of::<idx_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    -(1 as libc::c_int) as idx_t / -s
                                }) <= -(1 as libc::c_int) as libc::c_long - n)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s
                                }) + 0 as libc::c_int as idx_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s
                                        }) + 0 as libc::c_int as idx_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) + 0 as libc::c_int as idx_t) as libc::c_int
                            }) != 0 && s == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    n
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < n + 0 as libc::c_int as idx_t) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < n
                                        && (-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < n - 1 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as idx_t / s < n) as libc::c_int
                            }
                        }
                    } else {
                        if s == 0 as libc::c_int as libc::c_long {
                            0 as libc::c_int
                        } else {
                            if n < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        n
                                    }) + 0 as libc::c_int as idx_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                n
                                            }) + 0 as libc::c_int as idx_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    n
                                                }) + 0 as libc::c_int as idx_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            n
                                        }) + 0 as libc::c_int as idx_t) as libc::c_int
                                }) != 0 && n == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < s + 0 as libc::c_int as idx_t) as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as idx_t)
                                            < s - 1 as libc::c_int as libc::c_long) as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as idx_t / n < s) as libc::c_int
                                }
                            } else {
                                (-(1 as libc::c_int) as idx_t / s < n) as libc::c_int
                            }
                        }
                    }) != 0
                {
                    let (fresh18, _fresh19) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh18;
                    1 as libc::c_int
                } else {
                    let (fresh20, fresh21) = n.overflowing_mul(s);
                    *(&mut nbytes as *mut idx_t) = fresh20;
                    fresh21 as libc::c_int
                }) != 0
        }
    {
        xalloc_die();
    }
    pa = xrealloc(pa, nbytes as size_t);
    *pn = n;
    return pa;
}
#[no_mangle]
pub fn xzalloc(s: usize) -> Vec<u8> {
    vec![0; s]
}

#[no_mangle]
pub fn xizalloc(s: idx_t) -> Option<Vec<u8>> {
    let size: usize = s.try_into().ok()?; // Convert idx_t to usize safely
    let result = unsafe { xicalloc(size, 1) }; // Call xicalloc, assuming it returns a raw pointer
    if result.is_null() {
        return None; // Handle allocation failure
    }
    let vec = unsafe { Vec::from_raw_parts(result as *mut u8, size, size) }; // Convert raw pointer to Vec
    Some(vec)
}

#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut s: size_t) -> *mut libc::c_void {
    let ptr = calloc(n, s);
return check_nonnull(ptr);
}
#[no_mangle]
pub fn xicalloc(n: usize, s: usize) -> *mut libc::c_void {
    let total_size = n.checked_mul(s).expect("Overflow in allocation size");
    let mut vec = Vec::with_capacity(total_size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec); // Prevent Vec from deallocating the memory
    ptr
}

#[no_mangle]
pub fn xmemdup(p: &[u8]) -> Vec<u8> {
    let mut result = vec![0; p.len()];
    result.copy_from_slice(p);
    result
}

#[no_mangle]
pub fn ximemdup(p: &[u8]) -> Vec<u8> {
    let s = p.len();
    let mut new_vec = Vec::with_capacity(s);
    new_vec.copy_from_slice(p);
    new_vec
}

#[no_mangle]
pub fn ximemdup0(p: &[u8]) -> Vec<u8> {
    let s = p.len();
    let mut result = Vec::with_capacity(s + 1);
    result.extend_from_slice(p);
    result.push(0); // Null-terminate the string
    result
}

#[no_mangle]
pub fn xstrdup(string: &str) -> String {
    let length = string.len();
    let mut duplicated = Vec::with_capacity(length + 1);
    duplicated.extend_from_slice(string.as_bytes());
    duplicated.push(0); // Null-terminate the string
    String::from_utf8(duplicated).expect("Failed to convert to String")
}

