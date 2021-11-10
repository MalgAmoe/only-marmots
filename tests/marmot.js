const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3

const main = async() => {
  console.log('Starting test...')

  const provider = anchor.Provider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.Marmot

  const baseAccount = anchor.web3.Keypair.generate()

  const tx = await program.rpc.startMarmotCentral({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount]
  })

  console.log("Your transaction signature", tx)

  let accounts = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log('Gif count', accounts.totalGifs.toString())

  await program.rpc.addGif('https://media.giphy.com/media/U7E464W1x7bOkXU7tV/giphy.gif', {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey
    }
  })

  accounts = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log('Gif count', accounts.totalGifs.toString())
  console.log('Gif list', accounts.gifList)
}

const runMain = async () => {
  try {
    await main()
    process.exit(0)
  } catch(e) {
    console.error(e)
    process.exit(1)
  }
}

runMain()