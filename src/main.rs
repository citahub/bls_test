use bls_signatures::{hash, PrivateKey, verify, aggregate, Serialize};
use rand;

fn main() {
    let msg = vec![42u8];
    let msg_hash = hash(&msg);

    // user A
    let a_sk = PrivateKey::generate(&mut rand::thread_rng());
    println!("a_sk: {:?}", a_sk);
    let a_pk = a_sk.public_key();
    println!("a_pk: {:?}", a_pk);
    let a_sk_bytes = a_sk.as_bytes();
    println!("a_sk_bytes: {:?}, len {}", a_sk_bytes, a_sk_bytes.len());
    let a_pk_bytes = a_pk.as_bytes();
    println!("a_pk_bytes: {:?}, len {}", a_pk_bytes, a_pk_bytes.len());
    let a_sig = a_sk.sign(&msg);
    println!("a_sig: {:?}", a_sig);
    let a_sig_bytes = a_sig.as_bytes();
    println!("a_sig_bytes: {:?}, len {}", a_sig_bytes, a_sig_bytes.len());
    assert!(verify(&a_sig, &[msg_hash], &[a_pk.clone()]));

    // user B
    let b_sk = PrivateKey::generate(&mut rand::thread_rng());
    println!("b_sk: {:?}", b_sk);
    let b_pk = b_sk.public_key();
    println!("b_pk: {:?}", b_pk);
    let b_sig = b_sk.sign(&msg);
    assert!(verify(&b_sig, &[msg_hash], &[b_pk.clone()]));

    let agg_sig = aggregate(&[a_sig, b_sig]);
    let agg_sig_bytes = agg_sig.as_bytes();
    println!("agg_sig_bytes: {:?}, len {}", agg_sig_bytes, agg_sig_bytes.len());
    let right_pks = vec![a_pk.clone(), b_pk.clone()];
    let right1_pks = vec![b_pk.clone(), a_pk.clone()];
    let wrong_pks = vec![];
    let wrong1_pks = vec![a_pk.clone()];
    let wrong2_pks = vec![b_pk.clone()];
    let wrong3_pks = vec![a_pk.clone(), b_pk.clone(), b_pk.clone()];
    assert!(!verify(&agg_sig, &[msg_hash, msg_hash], &wrong_pks));
    assert!(!verify(&agg_sig, &[msg_hash, msg_hash], &wrong1_pks));
    assert!(!verify(&agg_sig, &[msg_hash, msg_hash], &wrong2_pks));
    assert!(!verify(&agg_sig, &[msg_hash, msg_hash], &wrong3_pks));

    assert!(verify(&agg_sig, &[msg_hash, msg_hash], &right_pks));
    assert!(verify(&agg_sig, &[msg_hash, msg_hash], &right1_pks));
}
