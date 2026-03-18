import conversation/key
import conversation/oid
import conversation/ref
import gleeunit/should

pub fn generate_produces_keypair_test() {
  // Generate should succeed and produce a keypair
  let kp = key.generate()
  let pub_key = key.public_key(kp)
  // Public key should be constructable
  case pub_key {
    key.Ed25519(_) -> should.be_true(True)
  }
}

pub fn sign_verify_roundtrip_test() {
  let kp = key.generate()
  let pub_key = key.public_key(kp)
  let message = <<"hello world":utf8>>
  let signature = key.sign(kp, message)
  key.verify(pub_key, message, signature) |> should.be_true()
}

pub fn verify_wrong_message_fails_test() {
  let kp = key.generate()
  let pub_key = key.public_key(kp)
  let signature = key.sign(kp, <<"correct":utf8>>)
  key.verify(pub_key, <<"wrong":utf8>>, signature) |> should.be_false()
}

pub fn verify_wrong_key_fails_test() {
  let kp1 = key.generate()
  let kp2 = key.generate()
  let pub_key2 = key.public_key(kp2)
  let signature = key.sign(kp1, <<"message":utf8>>)
  key.verify(pub_key2, <<"message":utf8>>, signature) |> should.be_false()
}

pub fn key_oid_deterministic_test() {
  let kp = key.generate()
  let pub_key = key.public_key(kp)
  let oid1 = key.oid(pub_key)
  let oid2 = key.oid(pub_key)
  // Same key always produces same oid
  oid.equals(ref.oid(oid1), ref.oid(oid2)) |> should.be_true()
}

pub fn different_keys_different_oids_test() {
  let kp1 = key.generate()
  let kp2 = key.generate()
  let oid1 = key.oid(key.public_key(kp1))
  let oid2 = key.oid(key.public_key(kp2))
  oid.equals(ref.oid(oid1), ref.oid(oid2)) |> should.be_false()
}
