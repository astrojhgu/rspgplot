pub mod types;
use types::*;
use std::os::raw::{c_char, c_int, c_float};
use std::ffi::CString;

#[link(name = "cpgplot")]
#[link(name = "pgplot")]
#[link(name = "X11")]
#[link(name = "gfortran")]
extern{
    fn cpgarro(x1:c_float, y1:c_float, x2:c_float, y2:c_float);
    fn cpgask(flag:c_int);
    fn cpgaxis(opt: *const c_char, 
        x1:c_float, y1:c_float, x2:c_float, y2:c_float, 
        v1:c_float, v2:c_float, step:c_float, nsub:c_int, 
        dmajl:c_float, dmajr:c_float, fmin:c_float, disp:c_float, orient:c_float);
    fn cpgband(mode:c_int, posn:c_int, xref:c_float, yref:c_float, x: *mut c_float, y: *mut c_float, ch_scalar: *mut c_char)->c_int;
    fn cpgbbuf();
    fn cpgbin(nbin:c_int, x:*const c_float, data:*const c_float, center:c_int);
    fn cpgbox(xopt: *const c_char, xtick:c_float, nxsub:c_int, yopt:*const c_char, ytick:c_float, nysub:c_int);
    fn cpgcirc(xcent:c_float, ycent:c_float, radius:c_float);
    fn cpgclos();
    fn cpgconb(a:*const c_float, idim:c_int, jdim:c_int, 
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c:*const c_float, nc:c_int, tr:*const c_float, blank:c_float);
    fn cpgconf(a:*const c_float, idim:c_int, jdim:c_int, 
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c1:c_float, c2:c_float, tr: *const c_float);
    fn cpgconl(a:*const c_float, idim:c_int, jdim:c_int, 
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c:c_float, tr:*const c_float, label:*const c_char, intval:c_int, minint:c_int);
    fn cpgcons(a:*const c_float, idim:c_int, jdim:c_int,
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c: *const c_float, nc:c_int, tr: *const c_float);
    fn cpgcont(a:*const c_float, idim:c_int, jdim:c_int, 
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c:*const c_float, nc:c_int, tr:*const c_float);
    fn cpgctab(l: *const c_float, r:*const c_float, g:*const c_float, b:*const c_float, nc:c_int, contra:c_float, bright:c_float);
    fn cpgcurs(x:*mut c_float, y:*mut c_float, ch_scalar:*mut c_char);
    fn cpgdraw(x:c_float, y:c_float);
    fn cpgebuf();
    fn cpgend();
    fn cpgenv(xmin:c_float, xmax:c_float, ymin:c_float, ymax:c_float, just:c_int, axis:c_int);
    fn cpgeras();
    fn cpgerr1(dir:c_int, x:c_float, y:c_float, e:c_float, t:c_float);
    fn cpgerrb(dir:c_int, n:c_int, x:*const c_float, y:*const c_float, e:*const c_float, t:c_float);
    fn cpgerrx(n: c_int, x1:*const c_float, x2:*const c_float, y: *const c_float, t:c_float);
    fn cpgerry(n: c_int, x:*const c_float, y1:*const c_float, y2:*const c_float, t:c_float);
    fn cpgetxt();
    fn cpggray(a:*const c_float, idim:c_int, jdim:c_int, i1:c_int, i2:c_int, j1:c_int, j2:c_int, fg:c_float, bg:c_float, tr:*const c_float);
    fn cpghi2d(data:*const c_float, nxv:c_int, nyv:c_int, 
        ix1:c_int, ix2:c_int, iy1:c_int, iy2:c_int, x:*const c_float, ioff:c_int, bias:c_float, center:c_int, ylims:*mut c_float);
    fn cpghist(n:c_int, data:*const c_float, datmin:c_float, datmax:c_float, nbin:c_int, pgflag:c_int);
    fn cpgiden();
    fn cpgimag(a:*const c_float, idim:c_int, jdim:c_int, i1:c_int, i2:c_int, j1:c_int, j2:c_int, a1:c_float, a2:c_float, tr:*const c_float);
    fn cpglab(xlbl:*const c_char, ylbl:*const c_char, toplbl:*const c_char);
    fn cpglcur(maxpt:c_int, npt: *mut c_int, x:*mut c_float, y:*mut c_float);
    fn cpgldev();    
    fn cpglen(units: c_int, string: *const c_char, xl: *mut c_float, yl: *mut c_float);
    fn cpgline(n:c_int, xpts:*const c_float, ypts:*const c_float);
    fn cpgmove(x:c_float, y:c_float);
    fn cpgmtxt(side:*const c_char, disp:c_float, coord:c_float, fjust:c_float, text:*const c_char);
    fn cpgncur(maxpt:c_int, npt:*mut c_int, x:*mut c_float, y:*mut c_float, symbol: c_int);

    fn cpgnumb(mm:c_int, pp:c_int, form:c_int, string:*mut c_char, string_length: *mut c_int);
    fn cpgolin(maxpt:c_int, npt:*mut c_int, x:*mut c_float, y:*mut c_float, symbol:c_int);
    fn cpgopen(device:*const c_char)->c_int;
    fn cpgpage();
    fn cpgpanl(nxc:c_int, nyc:c_int);
    fn cpgpap(width:c_float, aspect:c_float);
    fn cpgpixl(ia:*const c_int, idim:c_int, jdim:c_int, i1:c_int, i2:c_int, j1:c_int, j2:c_int, x1:c_float, x2:c_float, y1:c_float, y2:c_float);
    fn cpgpnts(n:c_int, x:*const c_float,y:*const c_float, symbol:*const c_int, ns:c_int);
    fn cpgpoly(n:c_int, xpts:*const c_float, ypts:*const c_float);
    fn cpgpt(n:c_int, xpts:*const c_float, ypts:*const c_float, symbol:c_int);
    fn cpgpt1(xpt:c_float, ypt:c_float, symbol:c_int);
    fn cpgptxt(x:c_float, y:c_float, angle:c_float, fjust:c_float, text:*const c_char);
    fn cpgqah(fs:*mut c_int, angle:*mut c_float, barb:*mut c_float);
    fn cpgqcf(font:*mut c_int);
    fn cpgqch(size:*mut c_float);
    fn cpgqci(ci:*mut c_int);
    fn cpgqcir(icilo:*mut c_int, icihi:*mut c_int);
    fn cpgqclp(state:*mut c_int);
    fn cpgqcol(ci1:*mut c_int, ci2:*mut c_int);
    fn cpgqcr(ci:c_int, cr:*mut c_float, cg:*mut c_float, cb:*mut c_float);
    fn cpgqcs(units:c_int, xch:*mut c_float, ych:*mut c_float);
    fn cpgqdt(n:c_int, tt:*mut c_char, type_length:*mut c_int, descr:*mut c_char, descr_length:*mut c_int, inter:*mut c_int);
    fn cpgqfs(fs:*mut c_int);
    fn cpgqhs(angle:*mut c_float, sepn:*mut c_float, phase:*mut c_float);
    fn cpgqid(id:*mut c_int);
    fn cpgqinf(item: *const c_char, value: *mut c_char, value_length: *mut c_int);
    fn cpgqitf(itf:*mut c_int);
    fn cpgqls(ls:*mut c_int);
    fn cpgqlw(lw:*mut c_int);
    fn cpgqndt(n:*mut c_int);
    fn cpgqpos(x:*mut c_float, y:*mut c_float);
    fn cpgqtbg(tbci:*mut c_int);
    fn cpgqtxt(x:c_float, y:c_float, angle:c_float, fjust:c_float, text:*const c_char, xbox:*mut c_float, ybox:*mut c_float);
    fn cpgqvp(units:c_int, x1:*mut c_float, x2:*mut c_float, y1:*mut c_float, y2:*mut c_float);
    fn cpgqvsz(units: c_int, x1:*mut c_float, x2:*mut c_float, y1:*mut c_float, y2:*mut c_float);
    fn cpgqwin(x1:*mut c_float, x2:*mut c_float, y1:*mut c_float, y2:*mut c_float);
    fn cpgrect(x1:c_float, x2:c_float, y1:c_float, y2:c_float);
    fn cpgrnd(x:c_float, nsub:*mut c_int)->c_float;
    fn cpgrnge(x1:c_float, x2:c_float, xlo:*mut c_float, xhi:*mut c_float);
    fn cpgsah(fs:c_int, angle:c_float, barb:c_float);
    fn cpgsave();
    fn cpgunsa();
    fn cpgscf(font:c_int);
    fn cpgsch(size:c_float);
    fn cpgsci(ci:c_int);
    fn cpgscir(icilo:c_int, icihi:c_int);
    fn cpgsclp(state:c_int);
    fn cpgscr(ci:c_int, cr:c_float, cg:c_float, cb:c_float);
    fn cpgscrl(dx:c_float, dy:c_float);
    fn cpgscrn(ci:c_int, name:*const c_char, ier:*mut c_int);
    fn cpgsfs(fs:c_int);
    fn cpgshls(ci:c_int, ch:c_float, cl:c_float, cs:c_float);
    fn cpgshs(angle:c_float, sepn:c_float, phase:c_float);
    fn cpgsitf(itf:c_int);
    fn cpgslct(id:c_int);
    fn cpgsls(ls:c_int);
    fn cpgslw(lw:c_int);
    fn cpgstbg(tbci:c_int);
    fn cpgsubp(nxsub:c_int, nysub:c_int);
    fn cpgsvp(xleft:c_float, xright:c_float, ybot:c_float, ytop:c_float);
    fn cpgswin(x1:c_float, x2:c_float, y1:c_float, y2:c_float);
    fn cpgtbox(xopt:*const c_char, xtick:c_float, nxsub:c_int, yopt:*const c_char, ytick:c_float, nysub:c_int);
    fn cpgtext(x:c_float, y:c_float, text:*const c_char);
    fn cpgtick(x1:c_float, y1:c_float, x2:c_float, y2:c_float, v:c_float, 
        tikl:c_float, tikr:c_float, disp:c_float, orient:c_float, text: *const c_char);
    fn cpgupdt();
    fn cpgvect(a:*const c_float, b:*const c_float, idim:c_int, jdim:c_int, 
        i1:c_int, i2:c_int, j1:c_int, j2:c_int, c:c_float, nc:c_int, tr:*const c_float, blank:c_float);
    fn cpgvsiz(xleft:c_float, xright:c_float, ybot:c_float, ytop:c_float);
    fn cpgvstd();
    fn cpgwedg(side:*const c_char, disp:c_float, width:c_float, fg:c_float, bg:c_float, label:*const c_char);
    fn cpgwnad(x1:c_float, x2:c_float, y1:c_float, y2:c_float);    
}

pub fn pgarro(x1:f32, y1:f32, x2:f32, y2:f32){
    unsafe{cpgarro(x1, y1, x2, y2)}
}

pub fn pgask(flag:bool){
    unsafe{cpgask(if flag {1} else {0})}
}

pub fn pgaxis(opt: &str, x1:f32, y1:f32, x2:f32, y2:f32, v1:f32, v2:f32, step:f32, nsub:usize, dmajl:f32, dmajr:f32, fmin:f32, disp:f32, orient:f32){
    let opt=CString::new(opt).unwrap();
    unsafe{cpgaxis(opt.as_ptr(), x1, y1, x2, y2, v1, v2, step, nsub as i32, dmajl, dmajr, fmin, disp, orient)}
}

pub fn pgband(mode:i32, posn:i32, xref:f32, yref:f32)->(f32, f32, c_char, i32){
    let mut x=0.0;
    let mut y=0.0;
    let mut ch_scalar=0;
    let r=unsafe{
        cpgband(mode, posn, xref, yref, &mut x, &mut y, &mut ch_scalar)
    };
    (x,y,ch_scalar,r)
}

pub fn pgbbuf(){
    unsafe{cpgbbuf()}
}

pub fn pgbin(x:&[f32], data:&[f32], center:bool){
    let nbin=x.len() as i32;
    assert_eq!(nbin, data.len() as i32);
    unsafe{
        cpgbin(nbin, x.as_ptr(), data.as_ptr(), if center {1} else {0})
    }
}


pub fn pgbox(xopt: &str, xtick:f32, nxsub:usize, yopt:&str, ytick:f32, nysub:usize){
    unsafe{
        cpgbox(CString::new(xopt).unwrap().as_ptr(), xtick, nxsub as i32, CString::new(yopt).unwrap().as_ptr(), ytick, nysub as i32);
    }
}

pub fn pgcirc(xcent:f32, ycent:f32, radius:f32){
    unsafe{
        cpgcirc(xcent, ycent, radius);
    }
}

pub fn pgclos(){
    unsafe{cpgclos()}
}

pub fn pgconb(a:&[f32], idim:usize, jdim:usize, 
    i1:usize, i2:usize, j1:usize, j2:usize, c:&[f32], tr:&[f32], blank:f32){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgconb(a.as_ptr(), idim as i32, jdim as i32, 
            i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c.as_ptr(), c.len() as i32, tr.as_ptr(), blank)
    }
}

pub fn pgconf(a:&[f32], idim:usize, jdim:usize, 
    i1:usize, i2:usize, j1:usize, j2:usize, c1:f32, c2:f32, tr: &[f32]){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgconf(a.as_ptr(), idim as i32, jdim as i32, 
            i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c1, c2, tr.as_ptr());
    }
}

pub fn pgconl(a:&[f32], idim:usize, jdim:usize, 
    i1:usize, i2:usize, j1:usize, j2:usize, c:f32, tr:&[f32], label:&str, intval:usize, minint:usize){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgconl(a.as_ptr(), idim as i32, jdim as i32, 
            i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c, tr.as_ptr(), CString::new(label).unwrap().as_ptr(), intval as i32, minint as i32)
    }
}

pub fn pgcons(a:&[f32], idim:usize, jdim:usize,
    i1:usize, i2:usize, j1:usize, j2:usize, c: &[f32], tr: &[f32]){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgcons(a.as_ptr(), idim as i32, jdim as i32,
            i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c.as_ptr(), c.len() as i32, tr.as_ptr())
    }
}

pub fn pgcont(a:&[f32], idim:usize, jdim:usize, 
    i1:usize, i2:usize, j1:usize, j2:usize, c:&[f32], tr:&[f32]){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgcont(a.as_ptr(), idim as i32, jdim as i32, 
            i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c.as_ptr(), c.len() as i32, tr.as_ptr())
    }
}

pub fn pgctab(l: &[f32], r:&[f32], g:&[f32], b:&[f32], contra:c_float, bright:c_float){
    unsafe{
        cpgctab(l.as_ptr(), r.as_ptr(), g.as_ptr(), b.as_ptr(), l.len() as i32, contra, bright);
    }
}

pub fn pgcurs(x:&mut f32, y:&mut f32)->c_char{
    let mut ch_scalar:c_char=0;
    unsafe{
        cpgcurs(x, y, &mut ch_scalar);
    }
    ch_scalar
}

pub fn pgdraw(x:f32, y:f32){
    unsafe{cpgdraw(x, y)}
}

pub fn pgebuf(){
    unsafe{cpgebuf()}
}

pub fn pgend(){
    unsafe{cpgend()}
}

pub fn pgenv(xmin:f32, xmax:f32, ymin:f32, ymax:f32, just:i32, axis:i32){
    unsafe{
        cpgenv(xmin, xmax, ymin, ymax, just, axis)
    }
}

pub fn pgeras(){
    unsafe{
        cpgeras()
    }
}

pub fn pgerr1(dir:i32, x:f32, y:f32, e:f32, t:f32){
    unsafe{
        cpgerr1(dir, x, y, e, t)
    }
}

pub fn pgerrb(dir:i32, x:&[f32], y:&[f32], e:&[f32], t:c_float){
    assert_eq!(x.len(), y.len());
    match dir{
        1..=4=>assert_eq!(x.len(), e.len()),
        5..=6=>assert_eq!(x.len()*2, e.len()),
        _=>panic!()
    }
    unsafe{
        cpgerrb(dir, x.len() as i32, x.as_ptr(), y.as_ptr(), e.as_ptr(), t)
    }
}

pub fn pgerrx(n: usize, x1:&[f32], x2:&[f32], y: &[f32], t:f32){
    unsafe{
        cpgerrx(n as i32, x1.as_ptr(), x2.as_ptr(), y.as_ptr(), t)
    }
}

pub fn pgerry(x:&[f32], y1:&[f32], y2:&[f32], t:f32){
    let n=x.len();
    assert_eq!(n, y1.len());
    assert_eq!(n, y2.len());
    unsafe{
        cpgerry(n as i32, x.as_ptr(), y1.as_ptr(), y2.as_ptr(), t)
    }
}

pub fn pgetxt(){
    unsafe{
        cpgetxt()
    }
}

pub fn pggray(a:&[f32], idim:usize, jdim:usize, i1:usize, i2:usize, j1:usize, j2:usize, fg:f32, bg:f32, tr:&[f32]){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpggray(a.as_ptr(), idim as i32, jdim as i32, i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, fg, bg, tr.as_ptr())
    }
}

pub fn pghi2d(data:&[f32], nxv:usize, nyv:usize, 
    ix1:usize, ix2:usize, iy1:usize, iy2:usize, x:&[f32], ioff:i32, bias:f32, center:bool){
    let mut ylims=vec![0_f32; (ix2-ix1+1) as usize];
    unsafe{
        cpghi2d(data.as_ptr(), nxv as i32, nyv as i32, 
            ix1 as i32, ix2 as i32, iy1 as i32, iy2 as i32, x.as_ptr(), ioff, bias, if center {1} else {0}, ylims.as_mut_ptr());       
    }
}

pub fn pghist(data:&[f32], datmin:f32, datmax:f32, nbin:usize, pgflag:i32){
    unsafe{
        cpghist(data.len() as i32, data.as_ptr(), datmin, datmax, nbin as i32, pgflag)
    }
}

pub fn pgiden(){
    unsafe{cpgiden()}
}

pub fn pgimag(a:&[f32], idim:usize, jdim:usize, i1:usize, i2:usize, j1:usize, j2:usize, a1:f32, a2:f32, tr:&[f32]){
    assert_eq!(tr.len(), 6);
    unsafe{
        cpgimag(a.as_ptr(), idim as i32, jdim as i32, i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, a1, a2, tr.as_ptr())
    }
}

pub fn pglab(xlbl:&str, ylbl:&str, toplbl:&str){
    unsafe{
        cpglab(CString::new(xlbl).unwrap().as_ptr(),CString::new(ylbl).unwrap().as_ptr(), CString::new(toplbl).unwrap().as_ptr())
    }
}

pub fn pglcur(npt: &mut usize, x:&mut [f32], y:&mut [f32]){
    let maxpt=x.len();
    assert_eq!(maxpt, y.len());
    let mut npt32=0;
    unsafe{
        cpglcur(maxpt as i32, &mut npt32, x.as_mut_ptr(), y.as_mut_ptr())
    }
    *npt=npt32 as usize;
}

pub fn pgldev(){
    unsafe{
        cpgldev()
    }
}

pub fn pglen(units: i32, string: &str)->(f32, f32){
    let mut xl=0.0;
    let mut yl=0.0;
    unsafe{
        cpglen(units, CString::new(string).unwrap().as_ptr(), &mut xl,&mut yl);
    }
    (xl,yl)
}

pub fn pgline(xpts:&[f32], ypts:&[f32]){
    let n=xpts.len();
    assert_eq!(n, ypts.len());
    unsafe{
        cpgline(n as i32, xpts.as_ptr(), ypts.as_ptr())
    }
}

pub fn pgmove(x:f32, y:f32){
    unsafe{
        cpgmove(x,y);
    }
}

pub fn pgmtxt(side:&str, disp:f32, coord:f32, fjust:f32, text:&str){
    unsafe{
        cpgmtxt(CString::new(side).unwrap().as_ptr(), disp, coord, fjust, CString::new(text).unwrap().as_ptr())
    }
}

pub fn pgncur(npt:&mut usize, x:&mut [f32], y:&mut [f32], symbol: i32){
    let maxpt=x.len();
    assert_eq!(maxpt, y.len());
    let mut npt32=0;
    unsafe{
        cpgncur(maxpt as i32, &mut npt32, x.as_mut_ptr(), y.as_mut_ptr(), symbol);
    }
    *npt=npt32 as usize;
}

pub fn pgnumb(mm:i32, pp:i32, form:i32)->String{
    let string=unsafe{CString::from_vec_unchecked(vec![0_u8;256])}.into_raw();
    let mut string_length=0;
    unsafe{
        cpgnumb(mm, pp, form, string, &mut string_length);
        CString::from_raw(string).into_string().unwrap()
    }
    
}

pub fn pgolin(npt:&mut usize, x:&mut [f32], y:&mut [f32], symbol:c_int){
    let maxpt=x.len();
    assert_eq!(maxpt, y.len());
    let mut npt32=0;
    unsafe{cpgolin(maxpt as i32, &mut npt32, x.as_mut_ptr(), y.as_mut_ptr(), symbol)}
    *npt=npt32 as usize
}

pub fn pgopen(device:&str)->i32{
    unsafe{
        cpgopen(CString::new(device).unwrap().as_ptr())
    }
}

pub fn pgpage(){
    unsafe{
        cpgpage()
    }
}

pub fn pgpanl(nxc:usize, nyc:usize){
    unsafe{cpgpanl(nxc as i32, nyc as i32)}
}

pub fn pgpap(width:f32, aspect:f32){
    unsafe{
        cpgpap(width, aspect)
    }
}

pub fn pgpixl(ia:&[i32], idim:usize, jdim:usize, i1:usize, i2:usize, j1:usize, j2:usize, x1:f32, x2:f32, y1:f32, y2:f32){
    unsafe{
        cpgpixl(ia.as_ptr(), idim as i32, jdim as i32, i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, x1, x2, y1, y2)
    }
}

pub fn pgpnts(x:&[f32],y:&[f32], symbol:&[i32]){
    unsafe{cpgpnts(x.len() as i32, x.as_ptr(),y.as_ptr(), symbol.as_ptr(), symbol.len() as i32)}
}

pub fn pgpoly(xpts:&[f32], ypts:&[f32]){
    unsafe{cpgpoly(xpts.len() as i32, xpts.as_ptr(), ypts.as_ptr())}
}

pub fn pgpt(xpts:&[f32], ypts:&[f32], symbol:i32){
    unsafe{
        cpgpt(xpts.len() as i32, xpts.as_ptr(), ypts.as_ptr(), symbol);
    }
}

pub fn pgpt1(xpt:f32, ypt:f32, symbol:i32){
    unsafe{
        cpgpt1(xpt, ypt, symbol);
    }
}

pub fn pgptxt(x:f32, y:f32, angle:f32, fjust:f32, text:&str){
    unsafe{
        cpgptxt(x, y, angle, fjust, CString::new(text).unwrap().as_ptr())
    }
}

pub fn pgqah()->ArrayHeadStyle{
    let mut fs=0;
    let mut angle=0.0;
    let mut barb=0.0;
    unsafe{
        cpgqah(&mut fs, &mut angle, &mut barb);
    }
    //(fs, angle, barb)
    let fill_style=match fs {
        1=>FillStyle::Filled,
        2=>FillStyle::Outline,
        _=>unreachable!()
    };
    ArrayHeadStyle{ fill_style, angle, barb}
}

pub fn pgqcf()->i32{
    let mut font=0;
    unsafe{
        cpgqcf(&mut font);
    }
    font
}

pub fn pgqch()->f32{
    let mut size=0.0;
    unsafe{
        cpgqch(&mut size);
    }
    size
}

pub fn pgqci()->i32{
    let mut ci=0;
    unsafe{
        cpgqci(&mut ci);
    }
    ci    
}

pub fn pgqcir()->(i32, i32){
    let mut icilo=0;
    let mut icihi=0;
    unsafe{
        cpgqcir(&mut icilo, &mut icihi);
    }
    (icilo, icihi)
}

pub fn pgqclp()->i32{
    let mut state=0;
    unsafe{
        cpgqclp(&mut state);
    }
    state
}

pub fn pgqcol()->(i32, i32){
    let mut ci1=0;
    let mut ci2=0;
    unsafe{
        cpgqcol(&mut ci1, &mut ci2);
    }
    (ci1, ci2)
}

pub fn pgqcr(ci:c_int)->(f32, f32, f32){
    let mut cr=0.0;
    let mut cg=0.0;
    let mut cb=0.0;
    unsafe{
       cpgqcr(ci, &mut cr, &mut cg, &mut cb);
    }
    (cr, cg, cb)
}

pub fn pgqcs(units:i32)->(f32, f32){
    let mut xch=0.0;
    let mut ych=0.0;
    unsafe{
        cpgqcs(units, &mut xch, &mut ych)
    }
    (xch, ych)
}

pub fn pgqdt(n:i32)->(String, String, bool){
    let tt=unsafe{CString::from_vec_unchecked(vec![0_u8;16])}.into_raw();
    let descr=unsafe{CString::from_vec_unchecked(vec![0_u8;128])}.into_raw();
    let mut type_length=0;
    let mut descr_length=0;
    let mut inter=0;
    unsafe{
        cpgqdt(n, tt, &mut type_length, descr, &mut descr_length, &mut inter);
    }
    let tt=unsafe{CString::from_raw(tt).into_string().unwrap()};
    let descr=unsafe{CString::from_raw(descr).into_string().unwrap()};
    (tt, descr, inter==1)
}

pub fn pgqfs()->i32{
    let mut fs=0;
    unsafe{
        cpgqfs(&mut fs);
    }
    fs
}

pub fn pgqhs()->HatchingStyle{
    let mut angle=0.0;
    let mut sepn=0.0;
    let mut phase=0.0;
    unsafe{
        cpgqhs(&mut angle, &mut sepn, &mut phase);
    }
    HatchingStyle{angle, sepn, phase}
}

pub fn pgqid()->i32{
    let mut id=0;
    unsafe{
        cpgqid(&mut id);
    }
    id
}

pub fn pgqinf(item: &str)->String{
    let mut value_length:i32=256;
    let item=CString::new(item).unwrap();
    let value=unsafe{CString::from_vec_unchecked(vec![0_u8;256])}.into_raw();
    unsafe{cpgqinf(item.as_ptr(), value, &mut value_length);
    CString::from_raw(value)}.into_string().unwrap()
}

pub fn pgqitf()->i32{
    let mut itf=0;
    unsafe{
        cpgqitf(&mut itf); 
    }
    itf
}

pub fn pgqls()->i32{
    let mut ls=0;
    unsafe{
        cpgqls(&mut ls);
    }
    ls
}

pub fn pgqlw()->i32{
    let mut lw=0;
    unsafe{
        cpgqlw(&mut lw)
    }
    lw
}

pub fn pgqndt()->i32{
    let mut n=0;
    unsafe{
        cpgqndt(&mut n)
    }
    n
}

pub fn pgqpos()->(f32, f32){
    let mut x=0.0;
    let mut y=0.0;
    unsafe{
        cpgqpos(&mut x, &mut y);
    }
    (x,y)
}

pub fn pgqtbg()->i32{
    let mut tbci=0;
    unsafe{
        cpgqtbg(&mut tbci);
    }
    tbci
}

pub fn pgqtxt(x:f32, y:f32, angle:f32, fjust:f32, text:&str)->([f32;4], [f32;4]){
    let mut xbox=[0.0_f32;4];
    let mut ybox=[0.0_f32;4];
    unsafe{
        cpgqtxt(x, y, angle, fjust, CString::new(text).unwrap().as_ptr(), xbox.as_mut_ptr(), ybox.as_mut_ptr());
    }
    (xbox, ybox)
}

pub fn pgqvp(units:i32)->(f32, f32, f32, f32){
    let mut x1=0.0;
    let mut x2=0.0;
    let mut y1=0.0;
    let mut y2=0.0;
    unsafe{
        cpgqvp(units, &mut x1, &mut x2, &mut y1, &mut y2);
    }
    (x1, x2, y1, y2)
}

pub fn pgqvsz(units: i32)->(f32, f32){
    let mut x1=0.0;
    let mut x2=0.0;
    let mut y1=0.0;
    let mut y2=0.0;
    unsafe{
        cpgqvsz(units, &mut x1, &mut x2, &mut y1, &mut y2);
    }
    (x2, y2)
}

pub fn pgqwin()->(f32, f32, f32, f32){
    let mut x1=0.0;
    let mut x2=0.0;
    let mut y1=0.0;
    let mut y2=0.0;
    unsafe{
        cpgqwin(&mut x1, &mut x2, &mut y1, &mut y2);
    }
    (x1, x2, y1, y2)
}

pub fn pgrect(x1:f32, x2:f32, y1:f32, y2:f32){
    unsafe{
        cpgrect(x1, x2, y1, y2)
    }
}

pub fn pgrnd(x:f32)->(i32, c_float){
    let mut nsub=0;
    let r=unsafe{
        cpgrnd(x, &mut nsub)
    };
    (nsub, r)
}

pub fn pgrnge(x1:f32, x2:f32)->(f32, f32){
    let mut xlo=0.0;
    let mut xhi=0.0;
    unsafe{
        cpgrnge(x1, x2, &mut xlo, &mut xhi);
    }
    (xlo, xhi)
}

pub fn pgsah(ah:ArrayHeadStyle){
    unsafe{
        cpgsah(match ah.fill_style{
            FillStyle::Filled=>1,
            FillStyle::Outline=>2,
        }, ah.angle, ah.barb)
    }
}

pub fn pgsave(){
    unsafe{
        cpgsave()
    }
}
pub fn pgunsa(){
    unsafe{
        cpgunsa()
    }
}

pub fn pgscf(font:i32){
    unsafe{
        cpgscf(font)
    }
}

pub fn pgsch(size:f32){
    unsafe{
        cpgsch(size)
    }
}

pub fn pgsci(ci:i32){
    unsafe{
        cpgsci(ci)
    }
}

pub fn pgscir(icilo:i32, icihi:i32){
    unsafe{
        cpgscir(icilo, icihi)
    }
}

pub fn pgsclp(state:i32){
    unsafe{
        cpgsclp(state)
    }
}

pub fn pgscr(ci:i32, cr:f32, cg:f32, cb:f32){
    unsafe{
        cpgscr(ci, cr, cg, cb)
    }
}

pub fn pgscrl(dx:f32, dy:f32){
    unsafe{
        cpgscrl(dx, dy)
    }
}

pub fn pgscrn(ci:i32, name:&str)->i32{
    let mut ier=0;
    unsafe{
        cpgscrn(ci, CString::new(name).unwrap().as_ptr(), &mut ier)
    }
    ier
}

pub fn pgsfs(fs:i32){
    unsafe{
        cpgsfs(fs)
    }
}

pub fn pgshls(ci:i32, ch:f32, cl:f32, cs:f32){
    unsafe{
        cpgshls(ci, ch, cl, cs)
    }
}

pub fn pgshs(hs:HatchingStyle){
    unsafe{
        cpgshs(hs.angle, hs.sepn, hs.phase)
    }
}

pub fn pgsitf(itf:i32){
    unsafe{
        cpgsitf(itf)
    }
}

pub fn pgslct(id:i32){
    unsafe{
        cpgslct(id)
    }
}

pub fn pgsls(ls:i32){
    unsafe{
        cpgsls(ls)
    }
}

pub fn pgslw(lw:i32){
    unsafe{cpgslw(lw)}
}

pub fn pgstbg(tbci:i32){
    unsafe{
        cpgstbg(tbci)
    }
}

pub fn pgsubp(nxsub:i32, nysub:i32){
    unsafe{
        cpgsubp(nxsub, nysub)
    }
}

pub fn pgsvp(xleft:f32, xright:f32, ybot:f32, ytop:f32){
    unsafe{
        cpgsvp(xleft, xright, ybot, ytop)
    }
}

pub fn pgswin(x1:f32, x2:f32, y1:f32, y2:f32){
    unsafe{
        cpgswin(x1, x2, y1, y2)
    }
}

pub fn pgtbox(xopt:&str, xtick:f32, nxsub:usize, yopt:&str, ytick:f32, nysub:usize){
    unsafe{
        cpgtbox(CString::new(xopt).unwrap().as_ptr(), xtick, nxsub as i32, CString::new(yopt).unwrap().as_ptr(), ytick, nysub as i32)
    }
}

pub fn pgtext(x:f32, y:f32, text:&str){
    unsafe{
        cpgtext(x, y, CString::new(text).unwrap().as_ptr())
    }
}

pub fn pgtick(x1:f32, y1:f32, x2:f32, y2:f32, v:f32, 
    tikl:f32, tikr:f32, disp:f32, orient:f32, text: &str){
    unsafe{
        cpgtick(x1, y1, x2, y2, v, 
            tikl, tikr, disp, orient, CString::new(text).unwrap().as_ptr());
    }
}

pub fn pgupdt(){
    unsafe{
        cpgupdt()
    }
}

pub fn pgvect(a:&[f32], b:&[f32], idim:usize, jdim:usize, 
    i1:usize, i2:usize, j1:usize, j2:usize, c:f32, nc:i32, tr:&[f32], blank:f32){
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), idim*jdim);
    assert_eq!(tr.len(),6);
    unsafe{
        cpgvect(a.as_ptr(), b.as_ptr(), idim as i32, jdim as i32, i1 as i32+1, i2 as i32, j1 as i32+1, j2 as i32, c, nc, tr.as_ptr(), blank)
    }
}

pub fn pgvsiz(xleft:f32, xright:f32, ybot:f32, ytop:f32){
    unsafe{
        cpgvsiz(xleft, xright, ybot, ytop)
    }
}

pub fn pgvstd(){
    unsafe{
        cpgvstd()
    }
}

pub fn pgwedg(side:&str, disp:f32, width:f32, fg:f32, bg:f32, label:&str){
    unsafe{
        cpgwedg(CString::new(side).unwrap().as_ptr(), disp, width, fg, bg, CString::new(label).unwrap().as_ptr())
    }
}

pub fn pgwnad(x1:f32, x2:f32, y1:f32, y2:f32){
    unsafe{
        cpgwnad(x1, x2, y1, y2)
    }
}

