import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import * as spl from "@solana/spl-token";
import * as web3 from "@solana/web3.js";
import { EventManager } from "../target/types/event_manager";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { assert } from "chai";

describe("Event Manager test suite", () => {
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

  type Metadata = {
    id: string; // max length 16
    name: string; // max length 40
    description: string; // max length 150
  };

  type Prices = {
    ticket: anchor.BN;
    sponsorship: anchor.BN;
  };

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
    const ticket = new anchor.BN(50 * 10 ** 2); // 50.00 with 2 decimal places
    const sponsorship = new anchor.BN(200 * 10 ** 2); // 200.00 with 2 decimal places

    const metadata: Metadata = {
      id,
      name,
      description,
    };

    const prices: Prices = {
      ticket,
      sponsorship,
    };

    // create new event
    const tx = await program.methods
      .newEvent(metadata, prices)
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
    assert.equal(infoEvent.metadata.name, "Festival Rio Babel");
    assert.equal(infoEvent.metadata.description, "Madrid 4 y 5 de Julio 2025");

    // check prices are what we expect
    assert.equal(infoEvent.prices.ticket.toNumber(), 50 * 10 ** 2);
    assert.equal(infoEvent.prices.sponsorship.toNumber(), 200 * 10 ** 2);

    // check that state is what we expect
    assert(infoEvent.state.isActive);
    assert.equal(infoEvent.state.activeSponsors.toNumber(), 0);
    assert.equal(infoEvent.state.uniqueSponsors.toNumber(), 0);
    assert.equal(infoEvent.state.ticketsSold.toNumber(), 0);
    assert.equal(infoEvent.state.sponsorshipsSold.toNumber(), 0);
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
    assert.equal(infoEvent.state.isActive, false);
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
