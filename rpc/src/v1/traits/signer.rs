// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Parity Signer-related rpc interface.
use jsonrpc_core::Error;

use v1::types::{U256, Bytes, TransactionModification, ConfirmationRequest, ConfirmationResponse, ConfirmationResponseWithToken};

build_rpc_trait! {
	/// Signer extension for confirmations rpc interface.
	pub trait Signer {

		/// Returns a list of items to confirm.
		#[rpc(name = "signer_requestsToConfirm")]
		fn requests_to_confirm(&self) -> Result<Vec<ConfirmationRequest>, Error>;

		/// Confirm specific request.
		#[rpc(name = "signer_confirmRequest")]
		fn confirm_request(&self, U256, TransactionModification, String) -> Result<ConfirmationResponse, Error>;

		/// Confirm specific request with token.
		#[rpc(name = "signer_confirmRequestWithToken")]
		fn confirm_request_with_token(&self, U256, TransactionModification, String) -> Result<ConfirmationResponseWithToken, Error>;

		/// Confirm specific request with already signed data.
		#[rpc(name = "signer_confirmRequestRaw")]
		fn confirm_request_raw(&self, U256, Bytes) -> Result<ConfirmationResponse, Error>;

		/// Reject the confirmation request.
		#[rpc(name = "signer_rejectRequest")]
		fn reject_request(&self, U256) -> Result<bool, Error>;

		/// Generates new authorization token.
		#[rpc(name = "signer_generateAuthorizationToken")]
		fn generate_token(&self) -> Result<String, Error>;

		/// Generates new web proxy access token.
		#[rpc(name = "signer_generateWebProxyAccessToken")]
		fn generate_web_proxy_token(&self) -> Result<String, Error>;
	}
}
