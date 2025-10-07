## Message exchange with post-quantum secure public key encryption

Many use cases of cryptography in day-to-day life fall under the umbrella of public key cryptography. Public key cryptography allows a person to publish a "public key" which others can use to encrypt a message to them, and when the message is recieved the original user has a private key that be used to decrypt the message, only their private key can be used for this, and as a result nobody that intercepts the message can read it. This allows real-time secure commnication between different parties over the internet.

At present most public key cryptography relies on the computational hardness of factorizing a number into primes. But in 1994 Peter Shor demonstrated that a number can be facotred into primes in polynomial time when using a quantum computer. This demands new cryptographic protocols that can withstand a brute-force attack from a quantum computer. The Open Quantum Safe project has developed [liboqs](https://github.com/open-quantum-safe/liboqs), an open-source library containing post-quantum secure cryptographic protocols.

This repositiory contains a rust program that will encrypt a text file using AES256 encryption and encapsulate the key with a post-quantum secure algorithm sourced from the liboqs library. The rust program will allow you to generate a private and public key pair, it will allow you encrypt a text file, or decrypted a message you have recieved.

This program requires the liboqs library.

This is a personal project I embarked on to practice rust and to explore the liboqs library. I would not use this for my bank information, but hey you do you.

