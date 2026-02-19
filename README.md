# Generating a valid ES384 PKCS#8 Key
```
openssl ecparam -name secp384r1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt
```

# Generating a valid RSA4096 PKCS#8 Key
```
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:4096 | openssl pkcs8 -topk8 -nocrypt
```