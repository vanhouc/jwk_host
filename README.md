# Generating a valid ES384 PKCS#8 Key
```
openssl ecparam -name secp384r1 -genkey -noout | openssl pkcs8 -topk8 -nocrypt
```
