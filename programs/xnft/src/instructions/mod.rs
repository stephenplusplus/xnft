// Copyright (C) 2022 Blue Coral, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod create_install;
mod create_permissioned_install;
mod create_review;
mod create_xnft;
mod delete_install;
mod delete_review;
mod grant_access;
mod revoke_access;
mod set_curator;
mod set_suspended;
mod transfer;
mod update_xnft;
mod verify_curator;

pub use create_install::*;
pub use create_permissioned_install::*;
pub use create_review::*;
pub use create_xnft::*;
pub use delete_install::*;
pub use delete_review::*;
pub use grant_access::*;
pub use revoke_access::*;
pub use set_curator::*;
pub use set_suspended::*;
pub use transfer::*;
pub use update_xnft::*;
pub use verify_curator::*;
