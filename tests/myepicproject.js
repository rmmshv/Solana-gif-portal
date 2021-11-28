const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async() => {
    console.log("🚀🚀🚀Starting test...");

    // Create and set the provider
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Myepicproject;

    // Create an account key pair
    const baseAccount = anchor.web3.Keypair.generate();

    let tx = await program.rpc.startStuffOff({
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
    });

    console.log("📝 📝 📝 Your transaction signature: ", tx);

    // Fetch data form the account
    let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifs.toString())

    // Call add_gif
    // Pass in the gif_link and the user submitting the link
    await program.rpc.addGif("https://media.giphy.com/media/hfT4eNf5EXeuY/giphy.gif", {
        accounts: {
            baseAccount: baseAccount.publicKey,
            user: provider.wallet.publicKey,
        },
    });

    // Fetch the account again to see new changes
    account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('👀 GIF Count', account.totalGifs.toString());

    // Access gif list on the account
    console.log('👀 GIF listL ', account.gifList);
}

const runMain = async() => {
    try {
        await main();
        process.exit(0);
    } catch (error) {
        console.error(error);
        process.exit(1);
    }
};

runMain();