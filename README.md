## Message exchange with post-quantum secure public key encryption

Many use cases of cryptography in day-to-day life fall under the umbrella of public key cryptography. Public key cryptography allows a person to publish a "public key" which others can use to encrypt a message to them, and when the message is recieved the original user has a private key that be used to decrypt the message, only their private key can be used for this, and as a result nobody that intercepts the message can read it. This allows real-time secure commnication between different parties over the internet.

At present most public key cryptography relies on the computational hardness of factorizing a number into primes. But in 1994 Peter Shor demonstrated that a number can be facotred into primes in polynomial time when using a quantum computer. This demands new cryptographic protocols that can withstand a brute-force attack from a quantum computer. The Open Quantum Safe project has developed [liboqs](https://github.com/open-quantum-safe/liboqs), an open-source library containing post-quantum secure cryptographic protocols.

This repositiory contains a rust program that will encrypt a text file using AES256 encryption and encapsulate the key with a post-quantum secure algorithm sourced from the liboqs library. The rust program will allow you to generate a private and public key pair, it will allow you encrypt a text file, or decrypted a message you have recieved.

This program relies on the [liboqs](https://github.com/open-quantum-safe/liboqs) library, see their github for more information.


## Installation and usage
Download the rust project and build it locally. There are three commands key-generation, encrypt, and decrypt, to generate public and private keys, encrypt files, and decrypt them. These can be run by cding into the root directory and running "cargo run -- key-generation" for example. Keys, encrypted files, and decrypted files will be written to the directory containing the rust project.

#### Note from author
This is a personal project I embarked on to practice rust and to explore the liboqs library, and as such this application has not recieved extensive vulnerability testing. I would not use this for my bank information, but hey you do you.

#### Future improvement
The biggest security priority is managing the sensitive information like keys, in particular making sure that any sensitive information would not be available if the application crashed, and that secret keys should be not written into loosely permissioned folders. For the user end it would be good to add options to name files when writing and to specify their locations.
