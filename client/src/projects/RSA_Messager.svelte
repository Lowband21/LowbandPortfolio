<script>
    import { onMount } from "svelte";

    let prime1 = "";
    let prime2 = "";
    let oddNumsTried;
    let valuesUsed;
    let loadingTime;
    let confidence;

    onMount(async () => {
        try {
            const startTime = performance.now();
            const response = await fetch("/generate_keys");
            const data = await response.json();
            const endTime = performance.now();
            loadingTime = (endTime - startTime).toFixed(2);
            prime1 = data.prime1;
            prime2 = data.prime2;
            oddNumsTried = data.oddNumbersTried;
            valuesUsed = data.valuesUsed;
            confidence = data.confidence.toFixed(10);
        } catch (error) {
            console.error("Failed to fetch prime numbers:", error);
        }
    });
</script>

<div id="content">
    <h2>
        Secure Messaging with RSA: A Deep Dive into Network Programming and
        Algorithm Optimization
    </h2>
    <p>
        The importance of secure communication in today's digital age cannot be
        overstated. To contribute to this crucial field, I developed a secure
        messaging system that employs RSA, a public-key cryptography protocol,
        to protect the messages exchanged between a client and a server.
    </p>
    <p>
        A standout aspect of this <a
            href="https://github.com/Lowband21/rsa_rt_messager"
            target="_blank"
            rel="noopener noreferrer">project</a
        > is the optimized generation of 2048-bit prime numbers. The prime numbers
        form an integral part of the RSA encryption algorithm, and their efficient
        generation significantly speeds up the key generation process. The system
        is capable of generating these primes in an impressive timeframe of approximately
        100 milliseconds on my local machine (slower on this website due to limited
        cloud performance).
    </p>
    <p>
        This level of optimization not only accelerates the process, but also
        ensures the robustness of the encryption, providing a secure yet highly
        responsive communication environment. It’s a delicate balance between
        speed and security, and this project skillfully maintains it.
    </p>
    <p>
        The successful execution of this project demonstrates a deep
        understanding of the RSA protocol, prime number theory, and algorithmic
        optimization. It also showcases how these principles can be applied in
        real-world network programming. By striking a balance between robust
        security measures and efficient performance, this project contributes
        towards shaping a safer and faster digital communication landscape. The
        lessons learned from this endeavor continue to inform and influence my
        approach to security, efficiency, and overall excellence in software
        development.
    </p>
    <p>Generated prime numbers (took {loadingTime}ms to generate):</p>
    <ul>
        <li>Prime 1: {prime1}</li>
        <li>Prime 2: {prime2}</li>
    </ul>
    <p>Statistics:</p>
    <ul>
        <li>Odd random numbers tried: {oddNumsTried}</li>
        <li>Values used to verify each prime: {valuesUsed}</li>
        <li>Confidence that each number is prime: {confidence}</li>
    </ul>
</div>

<style>
    ul {
        max-width: 100%;
        overflow-x: auto;
        white-space: normal;
        word-wrap: break-word;
    }
    li {
        display: block;
        margin-right: 10px;
        border: 1px solid #ccc;
        padding: 5px;
        background-color: #333333;
        color: white;
    }
</style>
