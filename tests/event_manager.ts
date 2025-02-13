import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import * as spl from "@solana/spl-token";
import * as web3 from "@solana/web3.js";
import { EventManager } from "../target/types/event_manager";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { assert } from "chai";

describe("Running: Event Manager test suite...", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.EventManager as Program<EventManager>;

  // accounts
  let organizer = provider.wallet as NodeWallet;
  let baseDenom: web3.PublicKey;

  // PDA
  let event: web3.PublicKey;
  let eventToken: web3.PublicKey;
  let ticketVault: web3.PublicKey;
  let sponsorshipVault: web3.PublicKey;

  // event id
  let id: string = Date.now().toString();

  before(async () => {
    // get event PDA
    [event] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(id), Buffer.from("event"), organizer.publicKey.toBuffer()],
      program.programId
    );
    console.log("> Event account:             ", event.toBase58());

    // get event token PDA
    [eventToken] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("event_token"), event.toBuffer()],
      program.programId
    );
    console.log("> Event Token account:       ", eventToken.toBase58());

    // get ticket vault PDA
    [ticketVault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ticket_vault"), event.toBuffer()],
      program.programId
    );
    console.log("> Ticket Vault account:      ", ticketVault.toBase58());

    // get sponsorship vault PDA
    [sponsorshipVault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sponsorship_vault"), event.toBuffer()],
      program.programId
    );
    console.log("> Sponsorship Vault account: ", sponsorshipVault.toBase58());

    baseDenom = await spl.createMint(
      provider.connection,
      organizer.payer,
      organizer.publicKey,
      organizer.publicKey,
      2
    );
  });

  it("Create a new event", async () => {
    const name: string = "Festival Rio Babel";
    const description: string = "Madrid 4 y 5 de Julio 2025";
    const ticket_price = new anchor.BN(50 * 10 ** 2); // 50.00 with 2 decimal places
    const sponsorship_price = new anchor.BN(200 * 10 ** 2); // 200.00 with 2 decimal places



    // create new event
    const tx = await program.methods
      .newEvent(id, name, description, ticket_price, sponsorship_price)
      .accounts({
        event,
        baseDenom,
        eventToken,
        ticketVault,
        sponsorshipVault,
        organizer: organizer.publicKey,
      })
      .rpc();

    await provider.connection.confirmTransaction(tx);

    // fetch event info struct from storage
    const infoEvent = await program.account.event.fetch(event);

    // check metadata is what we expect
    assert.equal(infoEvent.name, "Festival Rio Babel");
    assert.equal(infoEvent.description, "Madrid 4 y 5 de Julio 2025");

    // check prices are what we expect
    assert.equal(infoEvent.ticketPrice.toNumber(), 50 * 10 ** 2);
    assert.equal(infoEvent.sponsorshipPrice.toNumber(), 200 * 10 ** 2);

    // check that state is what we expect
    assert(infoEvent.isActive);
    assert.equal(infoEvent.activeSponsors.toNumber(), 0);
    assert.equal(infoEvent.uniqueSponsors.toNumber(), 0);
    assert.equal(infoEvent.ticketsSold.toNumber(), 0);
    assert.equal(infoEvent.sponsorshipsSold.toNumber(), 0);
  });

  it("End event", async () => {
    const tx = await program.methods
      .endEvent()
      .accounts({
        event,
        organizer: organizer.publicKey,
      })
      .rpc();

    await provider.connection.confirmTransaction(tx);

    const infoEvent = await program.account.event.fetch(event);
    assert.equal(infoEvent.isActive, false);
  });

  it("Remove event", async () => {
    const someInfoEvent = await program.account.event.fetchNullable(event);
    assert.notEqual(someInfoEvent, null);

    const tx = await program.methods
      .removeEvent()
      .accounts({
        event,
        sponsorshipVault,
        ticketVault,
        eventToken,
        organizer: organizer.publicKey,
      })
      .rpc();

    await provider.connection.confirmTransaction(tx);

    const infoEvent = await program.account.event.fetchNullable(event);
    assert.equal(infoEvent, null);
  });
});