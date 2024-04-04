//! The abilies used by the player in the game.
//! They may target units or target points in space.
//! The abilities are gathered from  https://github.com/Blizzard/s2client-api

use serde::{Deserialize, Serialize};

#[cfg(feature = "arrow")]
use arrow2_convert::{ArrowDeserialize, ArrowField, ArrowSerialize};

pub mod a;
pub use a::*;
pub mod b;
pub use b::*;
pub mod c;
pub use c::*;
pub mod d;
pub use d::*;
pub mod e;
pub use e::*;
pub mod f;
pub use f::*;
pub mod g;
pub use g::*;
pub mod h;
pub use h::*;
pub mod i;
pub use i::*;
pub mod k;
pub use k::*;
pub mod l;
pub use l::*;
pub mod m;
pub use m::*;
pub mod n;
pub use n::*;
pub mod o;
pub use o::*;
pub mod p;
pub use p::*;
pub mod q;
pub use q::*;
pub mod r;
pub use r::*;
pub mod s;
pub use s::*;
pub mod t;
pub use t::*;
pub mod u;
pub use u::*;
pub mod v;
pub use v::*;
pub mod w;
pub use w::*;
pub mod x;
pub use x::*;
pub mod y;
pub use y::*;
pub mod z;
pub use z::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(
    feature = "arrow",
    derive(ArrowField, ArrowSerialize, ArrowDeserialize)
)]
#[cfg_attr(feature = "arrow", arrow_field(type = "sparse"))]
pub enum Ability {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    K(K),
    L(L),
    M(M),
    N(N),
    O(O),
    P(P),
    Q(Q),
    R(R),
    S(S),
    T(T),
    U(U),
    V(V),
    W(W),
    X(X),
    Y(Y),
    Z(Z),
}

impl From<u16> for Ability {
    fn from(id: u16) -> Self {
        Self::from_id(id)
    }
}

impl Ability {
    pub fn from_id(id: u16) -> Self {
        if let Some(ability) = A::from_id(id) {
            return Self::A(ability);
        }
        if let Some(ability) = B::from_id(id) {
            return Self::B(ability);
        }
        if let Some(ability) = C::from_id(id) {
            return Self::C(ability);
        }
        if let Some(ability) = D::from_id(id) {
            return Self::D(ability);
        }
        if let Some(ability) = E::from_id(id) {
            return Self::E(ability);
        }
        if let Some(ability) = F::from_id(id) {
            return Self::F(ability);
        }
        if let Some(ability) = G::from_id(id) {
            return Self::G(ability);
        }
        if let Some(ability) = H::from_id(id) {
            return Self::H(ability);
        }
        if let Some(ability) = I::from_id(id) {
            return Self::I(ability);
        }
        if let Some(ability) = K::from_id(id) {
            return Self::K(ability);
        }
        if let Some(ability) = L::from_id(id) {
            return Self::L(ability);
        }
        if let Some(ability) = M::from_id(id) {
            return Self::M(ability);
        }
        if let Some(ability) = N::from_id(id) {
            return Self::N(ability);
        }
        if let Some(ability) = O::from_id(id) {
            return Self::O(ability);
        }
        if let Some(ability) = P::from_id(id) {
            return Self::P(ability);
        }
        if let Some(ability) = Q::from_id(id) {
            return Self::Q(ability);
        }
        if let Some(ability) = R::from_id(id) {
            return Self::R(ability);
        }
        if let Some(ability) = S::from_id(id) {
            return Self::S(ability);
        }
        if let Some(ability) = T::from_id(id) {
            return Self::T(ability);
        }
        if let Some(ability) = U::from_id(id) {
            return Self::U(ability);
        }
        if let Some(ability) = V::from_id(id) {
            return Self::V(ability);
        }
        if let Some(ability) = W::from_id(id) {
            return Self::W(ability);
        }
        if let Some(ability) = X::from_id(id) {
            return Self::X(ability);
        }
        if let Some(ability) = Y::from_id(id) {
            return Self::Y(ability);
        }
        if let Some(ability) = Z::from_id(id) {
            return Self::Z(ability);
        }
        tracing::error!("Unknown ability id: {}", id);
        Self::N(N::Null)
    }
}
