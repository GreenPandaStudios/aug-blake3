// aug-spec: "api.aug.md" explains this file. Read it before changes; refresh with aug spec.
import HashError from contracts
extern C _hash(Bytes input) returns string unless HashError
/** Return a lowercase 64-character BLAKE3 digest, computed by the Rust crate. */
hash(Bytes input) returns string:
    unsafe:
        return _hash(input)

test hash:
    when vectors:
        it hashes_abc:
            Bytes input = "abc".bytes()
            assert(hash(input) == "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85")
