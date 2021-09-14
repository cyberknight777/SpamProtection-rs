use spamprotection::info;

#[test]
fn test_ptid() {
    assert_eq!(
        info::full("@Pero_Sar1111").get_ptid(),
        "TEL-9b7ef46a550112edea3ba46220283b1bfd8feddd09b26888524ef7245947e97f-b105a169"
    );
}
