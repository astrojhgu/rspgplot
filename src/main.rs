extern crate rspgplot;

fn main() {
    let xs=vec![1.,2.,3.,4.,5.];
    let ys:Vec<_>=xs.iter().map(|&x| x*x).collect();
    rspgplot::pgopen(&"?");
    let (w,h)=rspgplot::pgqvsz(1);
    println!("{} {}", w, h);
    rspgplot::pgpap(15., 2.0/3.0);
    rspgplot::pgslw(10);
    rspgplot::pgenv(0.0, 6., -4.0, 30., 0, 0);
    
    rspgplot::pglab("(x)", "(y)", "PGPLOT Example 1:  y = x\\u2");
    rspgplot::pgpt(&xs, &ys, 2);
    rspgplot::pgsls(3);
    rspgplot::pgline(&xs, &ys);
    let mut x=0.0;
    let mut y=0.0;
    rspgplot::pgcurs(&mut x, &mut y);
    rspgplot::pgend();
}
