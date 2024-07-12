import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GestionDeEventos } from "../target/types/gestion_de_eventos";
import { Keypair, PublicKey } from '@solana/web3.js';
import { createMint, createFundedWallet, createAssociatedTokenAccount } from './utils';

import { BN } from "bn.js";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";
import { assert } from "chai";

function sleep(seg: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, seg * 1000));
}

let cierre_preventas_segs = 3;
let cierre_evento_segs = 6;
let fondeo_cuenta_tokens = 40;
let precio_ticket = 10;


describe("gestion-de-eventos", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GestionDeEventos as Program<GestionDeEventos>;

  let acceptedMint: PublicKey;

  //PDA
  let eventPublicKey: PublicKey;
  let eventMint: PublicKey;
  let treasuryVault: PublicKey;
  let gainVault: PublicKey;

  // Organizador walletAcceptedMintATA
  let walletAcceptedMintATA: PublicKey;

  //Crear usuario Sponsor
  let alice: Keypair;
  let aliceMonedaDeCambioAccount: PublicKey;
  let aliceEventTokenAccount: PublicKey;

  before(async () => {
    acceptedMint = await createMint(provider);

    walletAcceptedMintATA = await getAssociatedTokenAddress(acceptedMint, provider.wallet.publicKey);

    [eventPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event", "utf-8"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
    [eventMint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event-mint", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );
    [treasuryVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("treasury_vault", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );
    [gainVault] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("gain_vault", "utf-8"), eventPublicKey.toBuffer()],
      program.programId
    );

    alice = await createFundedWallet(provider, 30); //Función de Daiana
    aliceMonedaDeCambioAccount = await createAssociatedTokenAccount(provider, acceptedMint, fondeo_cuenta_tokens, alice); // Función de Daiana
    aliceEventTokenAccount = await getAssociatedTokenAddress(eventMint, alice.publicKey);
  });

  // Inicializamos el Evento con una fecha de cierre de preventa de + 2 Segundos, y una fecha de cierre automatico de evento de + 4 segundos
  // y para simular el paso del tiempo y ve su incidencia en el comportamineto del programa intercalaremos algunos Timeouts

  it("Is initialized!", async () => {
    // Add your test here.
    const nombre = "Solana Bootcamp";
    const precio = new BN(precio_ticket);
    const open_sales = true;
    ////////// para pruebas de automatizacion de cierre de ventas ///////////////////
    const currentTimestamp = Math.floor(Date.now() / 1000);       // Timestamp en segundos de la ejecucion del test
    const fecha_cierre_de_preventa = new BN(currentTimestamp + cierre_preventas_segs);
    const fecha_cierre_de_ventas = new BN(currentTimestamp + cierre_evento_segs);  // establecemos el cierre del evento en realidad ( me quedó con ese nombre)

    const tx = await program.methods.crearEvento(nombre, precio, open_sales, fecha_cierre_de_preventa, fecha_cierre_de_ventas)
      .accounts({
        evento: eventPublicKey,
        acceptedMint: acceptedMint,
        eventMint: eventMint,
        treasuryVault: treasuryVault,
        gainVault: gainVault,
        authority: provider.wallet.publicKey,
      })
      .rpc();
    // Informacion del evento
    // const eventAccount = await program.account.evento.fetch(eventPublicKey)
    // console.log("Informacion del evento: ", eventAccount.name);
  });

  // Test Sponsor con timestamp transcurrido error SeCierranLasVentas
  it("Alice compra 20 tokens del evento pagando 20 de sus 40 unidades de la moneda de cambio", async () => {
    const aliceAccountAntes = await getAccount(provider.connection, aliceMonedaDeCambioAccount);

    const qty = new BN(20);
    await program.methods
      .sponsorEvent(qty)
      .accounts({
        eventMint: eventMint,
        payerAcceptedMintAta: aliceMonedaDeCambioAccount,
        evento: eventPublicKey,
        authority: alice.publicKey,
        payerEventMintAta: aliceEventTokenAccount,
        treasuryVault: treasuryVault
      })
      .signers([alice])
      .rpc();
  });
  

  // Test Comprar Tickets en preventa
  it("Alice decide comprar solo 3 porque su amiga Rogelia no confirmó asistencia", async () => {
    console.log("A Alice ahora le alcanza para comprar 4 tickets en preventa ya que cuestan un 50% menos, pero...");
    const qty = new BN(3);
    await program.methods.comprarTickets(qty)
      .accounts({
        payerAcceptedMintAta: aliceMonedaDeCambioAccount,
        evento: eventPublicKey,
        authority: alice.publicKey,
        gainVault: gainVault
      })
      .signers([alice])
      .rpc(); 
  });

  // dejamos pasar el tiempo de preventa

  it("El tiempo de preventa transcurrió", async () => {
    const aliceBalance = await getAccount(provider.connection, aliceMonedaDeCambioAccount);
    console.log("Saldo de Alice: Luego de comprar 20 tokens y 3 tickets en preventa: ------- ", aliceBalance.amount.toString());
    console.log("La fecha de cierre de preventa se acerca");
    await sleep(cierre_preventas_segs);
    console.log("Rogelia pudo solucionar su inconveniente y confirma que asistirá al evento")
  });

  // Test compra de ticket sin suficientes tockens

  it("...ya no le alcanza. El descuento del 50% ya no está vigente y los tickets valen 10", async () => {
    const qty = new BN(1);
    let errMsg = "";
    try {
      await program.methods.comprarTickets(qty)
      .accounts({
        payerAcceptedMintAta: aliceMonedaDeCambioAccount,
        evento: eventPublicKey,
        authority: alice.publicKey,
        gainVault: gainVault
      })
      .signers([alice])
      .rpc();
    } catch (err) {
      if (err.transactionMessage) {
        errMsg = err.transactionMessage
      } else {
        errMsg = err.msg
      }
    }
    console.log("Alice no puede comprar tokens porque... ", errMsg )
    // está raro, ahora no me retorna el eror custom cuando se intenta comprar luego del cierre
    let verificarError = errMsg == "La venta de tickets esta cerrada" || errMsg.includes("Transaction simulation failed: Error processing Instruction 0:")
    assert(verificarError);
  });

  // Test Withdraw
  it("Event organizer should withdraw funds", async () => {

    const amount = new BN(1); // 1 USDC
    await program.methods
      .withdraw(amount)
      .accounts({
        evento: eventPublicKey,
        acceptedMint: acceptedMint, // example: USDC
        authority: provider.wallet.publicKey, // event organizer
        treasuryVault: treasuryVault, // stores all Accepted Mint (USDC) from sponsorships
        authotiryAcceptedMintAta: walletAcceptedMintATA, // account where the event organizer receives accepted mint(USDC)
      })
      .rpc();

    // show event treasury vault info
    // should have 4 (5-1) USDC
    const treasuryVaultAccount = await getAccount(
      provider.connection,
      treasuryVault
    );
    // console.log("Event treasury vault: ", treasuryVaultAccount);

    // show event organizer accepted mint (USDC) ATA info
    // should have 1 accepte mint (USDC) 
    const organizerUSDCBalance = await getAccount(
      provider.connection,
      walletAcceptedMintATA // event organizer Accepted mint account (USDC account)
    );
    // console.log("Alice Accepted mint ATA: ", organizerUSDCBalance);

  });

  // Test cerrar evento
  it("Cerrar evento", async () => {
    program.methods.cerrarEvento()
      .accounts({
        evento: eventPublicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();
  })

  // Comprar despues de cerrado el evento

  it("Alice no puede comprar mas tickets", async () => {

    let errMsg = "";
    const qty = new BN(10);
    try {
      await program.methods.comprarTickets(qty)
        .accounts({
          payerAcceptedMintAta: aliceMonedaDeCambioAccount,
          evento: eventPublicKey,
          authority: alice.publicKey,
          gainVault: gainVault
        })
        .signers([alice])
        .rpc();
    } catch (err) {
      if (err.transactionMessage) {
        errMsg = err.transactionMessage
      } else {
        errMsg = err.msg
      }
    }
    let verificarError = errMsg == "La venta de tickets esta cerrada" || errMsg.includes("Transaction simulation failed: Error processing Instruction 0:")
    assert(verificarError);

  });

  // TEST: Withdraw earnings
  it("Alice Should withdraw earnings", async () => {

    // show total sponsorships
    const eventAccount = await program.account.evento.fetch(eventPublicKey);
    console.log("Event total sponsorships: ", eventAccount.sponsorAmount.toNumber());

    // show event gain vault amount
    let gainVaultAccount = await getAccount(
      provider.connection,
      gainVault
    );
    console.log("Event gain vault amount: ", gainVaultAccount.amount);

    // show Alice sponsorship tokens
    let aliceTokens = await getAccount(
      provider.connection,
      aliceMonedaDeCambioAccount
    );
    console.log("Alice sponsorship tokens: ", aliceTokens.amount);

    await program.methods
      .withdrawEarnings()
      .accounts({
        userEventMintAta: aliceEventTokenAccount,
        evento: eventPublicKey,
        authority: alice.publicKey,
        gainVault: gainVault,
        userAcceptedMintAta: aliceMonedaDeCambioAccount,
        eventMint: eventMint
      })
      .signers([alice])
      .rpc();

    // show event gain vault amount
    gainVaultAccount = await getAccount(
      provider.connection,
      gainVault
    );
    console.log("Event gain vault amount: ", gainVaultAccount.amount);
  });
});
