import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GestionDeEventos } from "../target/types/gestion_de_eventos";
import { Keypair, PublicKey } from '@solana/web3.js';
import { createMint } from './utils';

import { BN } from "bn.js";

describe("gestion-de-eventos", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GestionDeEventos as Program<GestionDeEventos>;

  // const name: String = "Solana Bootcamp";
  // const precio = new BN(1);
  // const fecha_cierre_de_ventas = new BN(1720457100);

  //
  let acceptedMint: PublicKey;

  //PDA
  let eventPublicKey: PublicKey;
  let eventMint: PublicKey;
  let treasuryVault: PublicKey;
  let gainVault: PublicKey;

  before(async () => {
    acceptedMint = await createMint(provider);

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

  });

  it("Is initialized!", async () => {
    // Add your test here.
    const nombre = "Solana Bootcamp";
    const precio = new BN(1);
    const fecha_cierre_de_ventas = new BN(1720467900); //Mon Jul 08 2024 19:45:00 GMT+0000
    const tx = await program.methods.crearEvento(nombre, precio, fecha_cierre_de_ventas)
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
    const eventAccount = await program.account.evento.fetch(eventPublicKey)
    console.log("Informacion del evento: ", eventAccount.name);
  });
});
