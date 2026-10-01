import HashError and hash from "https://github.com/GreenPandaStudios/aug-blake3#v0.1.1"
try:
    Bytes input = "abc".bytes()
    print(value=hash(input))
catch HashError error:
    print(value=error.message)
