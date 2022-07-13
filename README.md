# whichSystem

![](https://img.shields.io/badge/made%20with-Rust-orange)

### Description
This program determines the Operative System of the target of your choice via its ttl.
It attempts to perform a ping, if the host is alive it will get its ttl value and output which OS is using,
otherwise it will output an error message.

---

### Table of Content
* Usage
* Contribute

---

### Usage
 What follows is an explanation of how to use this program

#### Compilation
The binary is already compiled and can be found in `target/debug/which_system`

---

#### Execution
This program only takes one argument, the address of the target whether its an URL or an IP
```bash
./which_system <ip addr>
```

---

### Contribute
I am open to suggestions, in case there are bugs lying around or if there are ways to improve the code :)
