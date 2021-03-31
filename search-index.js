var searchIndex = JSON.parse('{\
"pow_sha256":{"doc":"MCaptch\'s SHA256 based Proof of Work library","i":[[3,"PoW","pow_sha256","Proof of Work over concrete type T. T can be any type …",null,null],[12,"nonce","","",0,null],[12,"result","","",0,null],[3,"PoWBuilder","","Builder for <code>PoW</code>.",null,null],[11,"nonce","","",1,[[["u64",15]]]],[11,"result","","",1,[[["string",3]]]],[11,"build","","Builds a new <code>PoW</code>.",1,[[],[["result",4],["pow",3],["string",3]]]],[3,"Config","","Configuration for generting proof of work Please choose a …",null,null],[12,"salt","","",2,null],[3,"ConfigBuilder","","Builder for <code>Config</code>.",null,null],[11,"salt","","",3,[[["string",3]]]],[11,"build","","Builds a new <code>Config</code>.",3,[[],[["result",4],["config",3],["string",3]]]],[11,"prove_work","","Create Proof of Work over item of type T.",2,[[["u32",15]],[["result",6],["pow",3]]]],[11,"prove_work_serialized","","Create Proof of Work on an already serialized item of …",2,[[["u32",15]],["pow",3]]],[11,"calculate","","Calculate the PoW score with the provided input T.",2,[[["pow",3]],[["u128",15],["result",6]]]],[11,"calculate_serialized","","Calculate the PoW score of an already serialized T and …",2,[[["pow",3]],["u128",15]]],[11,"is_valid_proof","","Verifies that the PoW is indeed generated out of the …",2,[[["pow",3]],["bool",15]]],[11,"is_sufficient_difficulty","","Checks if the PoW result is of sufficient difficulty",2,[[["u32",15],["pow",3]],["bool",15]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"to_owned","","",1,[[]]],[11,"clone_into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"to_owned","","",2,[[]]],[11,"clone_into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"to_owned","","",3,[[]]],[11,"clone_into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"clone","","",1,[[],["powbuilder",3]]],[11,"clone","","",0,[[],["pow",3]]],[11,"clone","","",3,[[],["configbuilder",3]]],[11,"clone","","",2,[[],["config",3]]],[11,"default","","",1,[[],["powbuilder",3]]],[11,"default","","",3,[[],["configbuilder",3]]],[11,"eq","","",0,[[["pow",3]],["bool",15]]],[11,"ne","","",0,[[["pow",3]],["bool",15]]],[11,"eq","","",2,[[["config",3]],["bool",15]]],[11,"ne","","",2,[[["config",3]],["bool",15]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",2,[[["formatter",3]],["result",6]]],[11,"serialize","","",0,[[],["result",4]]],[11,"serialize","","",2,[[],["result",4]]],[11,"deserialize","","",0,[[],["result",4]]],[11,"deserialize","","",2,[[],["result",4]]]],"p":[[3,"PoW"],[3,"PoWBuilder"],[3,"Config"],[3,"ConfigBuilder"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);