extern crate linked_hash_map;
extern crate yaml_rust;
use yaml_rust::yaml::YamlLoader;
use yaml_rust::Yaml;
use yaml_rust::emitter::YamlEmitter;

fn take_data_off(tuple: (&str, &str)) {
    use std::fs::File;
    let (in_filename, out_filename) = tuple;
    let in_yaml = {
        use std::io::Read;
        let mut s = String::new();
        let mut file = File::open(in_filename)
            .unwrap();
        let _ = file.read_to_string(&mut s);
        s
    };
    let in_yaml = YamlLoader::load_from_str(&in_yaml)
        .unwrap();
    let in_yaml = &in_yaml[0];

    let mut out_yaml = linked_hash_map::LinkedHashMap::new();
    for (ikey, ivalue) in in_yaml.as_hash().unwrap().iter() {
        if ikey != &Yaml::String("args".to_string()) {
            out_yaml.insert(ikey.clone(), ivalue.clone());
        } else {
            let mut args_vec = yaml_rust::yaml::Array::new();
            for index in 0.. {
                let mut out_args_hash = linked_hash_map::LinkedHashMap::<Yaml, Yaml>::new();
                let yaml = &ivalue[index];
                if yaml.is_badvalue() {
                    break;
                }
                let hash = yaml
                    .as_hash()
                    .unwrap();
                for (k, v) in hash.iter() {
                    if k != &Yaml::String("DATA".to_string()) {
                        out_args_hash.insert(k.clone(), v.clone());
                    }
                }
                if !out_args_hash.is_empty() {
                    let _ = args_vec.push(Yaml::Hash(out_args_hash.clone()));
                }
            }
            out_yaml.insert(ikey.clone(), Yaml::Array(args_vec));
        }
    }

    let out_yaml = Yaml::Hash(out_yaml);
    
    let mut out_yaml_string = String::new();
    let _ = {
        let mut emitter = YamlEmitter::new(&mut out_yaml_string);
        let _ = emitter
            .dump(&out_yaml)
            .unwrap();
    };
    out_yaml_string += "\r\n";

    let _ = {
        use std::io::Write;
        let mut file = File::create(out_filename)
            .unwrap();
        let _ = file
            .write(out_yaml_string.as_bytes())
            .unwrap();
        ()
    };
}

fn main() {
    let paths = {
        let mut v = Vec::new();
        v.push(("blockcounter/en_base.yml", "blockcounter/en.yml"));
        v.push(("showblock/en_base.yml",    "showblock/en.yml"));
        v
    };
    for tuple in paths {
        take_data_off(tuple);
    }
}
