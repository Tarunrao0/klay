import mongoose, { Schema } from "mongoose";

// Schema for proposal with voting options
const proposalSchema: Schema = new Schema({
  proposalId: { type: String, required: true },
  createrName: { type: String, required: true },
  proposalDescription: { type: String, required: true },
  createrPubKey: { type: String, required: true },
  yesVotes: { type: Number, default: 0 },
  noVotes: { type: Number, default: 0 },
  createdAt: { type: Date, default: Date.now },
});

// Export or create the model if it doesn't exist
const Proposal =
  mongoose.models.Proposal || mongoose.model("Proposal", proposalSchema);

export default Proposal;
