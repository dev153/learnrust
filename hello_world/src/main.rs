
fn print_point(pnt: &Point) {
    println!("{:#?}",pnt);
}

fn inc_x(pnt: &mut Point) {
    pnt.x += 1;
}

fn inc_y(pnt: &mut Point) {
    pnt.y += 1;
}

#[derive(Clone,Debug)]
struct Point {
    x: i32,
    y: i32
}

fn max(a: i32, b:i32)->i32{
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let hellostr: &str = "Hello";
    let name = "world";
    let booltest: bool = true;
    println!("{}: {}, {}!", booltest, hellostr, name);
    let num1: i32 = 24;
    let num2: i32 = 42;
    if num1 > num2 {
        println!("{} is greater than {}", num1, num2);
    } else {
        println!("{} is less than {}", num1, num2);
    }

    let mut a = 30;
    let mut b = 45;
    print!("The gcd of {} and {} is ", a, b);
    while b != 0 {
        let temp = b;
        b = b % a;
        a = temp;
    }
    println!("{}", a);

    println!("The maximum of {} and {} is {}", a, b, max(a,b));
    let the_point = Point {x:24,y:42};
    println!("the_point: x={},y={}",the_point.x,the_point.y);
    let another_point = Point { y:1, x:2};
    println!("another_point: x={},y={}",another_point.x,another_point.y);
    println!("{:#?}",another_point); // pretty-print value

    let p1 = Point{x:1,y:2};
    println!("{:#?}",p1);
    let p2 = &p1;
    println!("{:#?}",p1);
    println!("{:#?}",p2);
    print_point(p2);
    let mut p3 = p1.clone();
    println!("==============");
    print_point(&p1);
    print_point(&p3);
    p3.x = 14;
    print_point(&p1);
    print_point(&p3);
    println!("==============");
    let mut p4 = Point{x:0,y:0};
    print_point(&p4);
    inc_x(&mut p4);
    inc_y(&mut p4); 
    print_point(&p4);
    let p5 = &mut p4;
    p5.x = 2;
    p5.y = 3;
    print_point(&p5);
}
