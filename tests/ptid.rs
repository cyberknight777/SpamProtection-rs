use spamprotectionbot_rs::info;

#[test]
fn test_ptid() {
    assert_eq!(info::full("@Pero_Sar1111").results.private_telegram_id, "TEL-9b7ef46a550112edea3ba46220283b1bfd8feddd09b26888524ef7245947e97f-b105a169");
}

