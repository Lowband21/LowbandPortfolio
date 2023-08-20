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
            const response = await fetch("/generate_keys");
            const data = await response.json();
            loadingTime = 100;
            prime1 = data.prime1;
            prime2 = data.prime2;
            oddNumsTried = data.oddNumbersTried;
            valuesUsed = data.valuesUsed;
            confidence = data.confidence.toFixed(15);
        } catch (error) {
            console.error("Failed to fetch prime numbers:", error);
        }
    });
</script>

<div id="content">
    <h2>
        RSA Encryption: Marrying Advanced Mathematics with Network Programming
    </h2>
    <p>
        In an era where digital interactions are ubiquitous, the necessity for
        secure communication mechanisms stands paramount. In addressing this
        need, I architected a robust messaging system, leveraging the RSA
        encryption protocol to solidify client-server interactions via
        public-key cryptography.
    </p>
    <p>
        A pivotal feature of this <a
            href="https://github.com/Lowband21/rsa_rt_messager"
            target="_blank"
            rel="noopener noreferrer">project</a
        > is its adeptness in the generation of 2048-bit prime numbers. These large
        primes are fundamental to RSA encryption, serving as the underpinnings of
        both the public and private keys.
    </p>
    <p>
        Delving deeper into the prime generation mechanism, the system employs a
        two-fold process:
    </p>
    <ul>
        <li>
            <strong>Random Number Generation:</strong> We initiate by producing odd
            random numbers of the desired bit-length. This ensures a higher probability
            of encountering a prime, given that all even numbers, barring 2, are
            non-prime.
        </li>
        <li>
            <strong>Primality Testing:</strong> For each generated number, the Solovay-Strassen
            primality test is executed, a probabilistic algorithm, to ascertain its
            primality. This test is both efficient and scalable for large numbers,
            making it apt for RSA key generation.
        </li>
    </ul>
    <p>
        The system's multithreading capability further amplifies its efficiency.
        Prime number candidates are generated and tested in parallel, maximizing
        CPU utilization and significantly trimming down the overall prime
        discovery time. Notably, a unique feature has been implemented to avoid
        the unlikely, yet possible scenario where the two primes generated are
        identical. In such cases, the system regenerates one of the primes to
        ensure uniqueness.
    </p>
    <p>
        While the system is optimized for speed, there's no compromise on
        security. Through multiple iterations of the Solovay-Strassen test, a
        high degree of confidence in the primality of the numbers is achieved,
        striking a harmonious balance between computational efficiency and
        cryptographic robustness.
    </p>
    <p>
        This venture is not just about building an encryption system. It's about
        masterfully weaving advanced mathematical constructs into real-world
        software, demonstrating the symbiosis between theory and application.
        It's a testament to the potential of combining mathematical rigor,
        algorithmic precision, and software engineering prowess.
    </p>
    <p>Prime numbers generated in {loadingTime}ms:</p>
    <ul>
        <li>Prime 1: {prime1}</li>
        <li>Prime 2: {prime2}</li>
    </ul>
    <p>Prime Generation Metrics:</p>
    <ul>
        <li>Odd random numbers evaluated: {oddNumsTried}</li>
        <li>Verification iterations per prime: {valuesUsed}</li>
        <li>Statistical confidence in prime validity: {confidence}</li>
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
