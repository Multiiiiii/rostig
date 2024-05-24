#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
   let width1 = 30;
   let height1 = 50;

   println!(
       "The area of the rectangle is {} square pixels.",
       area(width1, height1)
   );

   let rect1 = (30, 60);
   println!(
       "The area of the rectangle is {} square pixels.",
       areatup(rect1)
   );

   let rect2 = Rectangle {
       width: 30,
       height: 70,
   };
   println!("rect2 is {:#?}", rect2);
   println!(
       "The area of the rectangle is {} square pixels.",
       areastruct(&rect2)
   );

   let scale =2;
   let rect3 = Rectangle {
       width: dbg!(30 * scale),
       height: 50,
   };
   dbg!(&rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn areatup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn areastruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}