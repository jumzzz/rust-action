use libactionkv::ActionKV;                // <1>
use std::collections::HashMap;


#[cfg(target_os = "windows")]             // <2>
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;


fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}


fn main() {

  const INDEX_KEY: &ByteStr = b"+index";

  let args: Vec<String> = std::env::args().collect();
  
  let fname = args.get(1).expect(&USAGE);
  let action = args.get(2).expect(&USAGE).as_ref();
  let key = args.get(3).expect(&USAGE).as_ref();
  
  // This will not throw an out of bounds but rather return 
  // a None in case no value was retrieved. 
  
  // You have to explicitly declare an .expect if you want
  // this to be a mandatory command-line argument.

  // Get returns an Option that's why
  let maybe_value = args.get(4);

  println!("Number of arguments = {}", args.len());

  let path = std::path::Path::new(&fname);

  // Listing 7.10 Initializing libactionkv::ActionKV
  let mut a = ActionKV::open(path).expect("unable to open file");
  a.load().expect("unable to load data");
  
  match action {
    "get" => {
      let index_as_bytes = a.get(&INDEX_KEY)
                                    .unwrap()
                                    .unwrap();

      let index_decoded = bincode::deserialize(&index_as_bytes);

      let index: HashMap<ByteString, u64> = index_decoded.unwrap();

      match index.get(key) {
        None => eprintln!("{:?} not found", key),
        Some(&i) => {
          let kv = a.get_at(i).unwrap();
          println!("{:?}", kv.value); 
        }
      }
    }

    "delete" => a.delete(key).unwrap(),

    "insert" => {
      let value = maybe_value.expect(&USAGE).as_ref();
      a.insert(key, value).unwrap();
      store_index_on_disk(&mut a, INDEX_KEY);
    }

    "update" => {
      let value = maybe_value.expect(&USAGE).as_ref();
      a.update(key, value).unwrap();
      store_index_on_disk(&mut a, INDEX_KEY);
    }
    _ => eprintln!("{}", &USAGE),
  }
}
