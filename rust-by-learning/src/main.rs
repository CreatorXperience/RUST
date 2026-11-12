// signed -> i8,i16,i32,i64,i128,isize
// unsigned -> u8,u16,u32,u64,u128,isize
// floating point number -> f32 f64[default]
// bool -> true or false
// char -> char for 4 bytes characters like "a", "&"
// the unit type (), whose only possible value is an empty tuple
//
mod expression;
mod tupl;
mod formatter;

fn main(){
  let logical:bool = true;
  let a_float:f64 = 1.7;

  let an_integer = 5i32;

  let default_float = 2.5; //f64
  let default_integer = 1;

  let mut inferred_type = 12; // type is inferred from another default_integer
  inferred_type = 4294967296i64;

  let mut mutable = 12;
  mutable = 21; // i32 by default
               //
               // mutable = true  -> error


  let mutable =  true; // variables can be overwritten with shadowing
                      //
  let array: [i32; 5] = [0,0,0,0,0];
  let tuple = (5u16, 0.5f32, true, -5.0f32);

  expression::express();
  tupl::reverse((64i32, true));
  formatter::format();

  }



