use cose::test_setup as test;
use cose::nss as nss;

#[test]
fn test_rfc6979_test_vector_1() {
    test::setup();
    // With SHA-256, message = "sample":
    // k = A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60
    // r = EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716
    // s = F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8
    // (the signature is just the bytes of r followed by the bytes of s)
    let signature =
        vec![0xef, 0xd4, 0x8b, 0x2a, 0xac, 0xb6, 0xa8, 0xfd, 0x11, 0x40, 0xdd, 0x9c, 0xd4, 0x5e,
             0x81, 0xd6, 0x9d, 0x2c, 0x87, 0x7b, 0x56, 0xaa, 0xf9, 0x91, 0xc3, 0x4d, 0x0e, 0xa8,
             0x4e, 0xaf, 0x37, 0x16,
             0xf7, 0xcb, 0x1c, 0x94, 0x2d, 0x65, 0x7c, 0x41, 0xd4, 0x36, 0xc7, 0xa1, 0xb6, 0xe2,
             0x9f, 0x65, 0xf3, 0xe9, 0x00, 0xdb, 0xb9, 0xaf, 0xf4, 0x06, 0x4d, 0xc4, 0xab, 0x2f,
             0x84, 0x3a, 0xcd, 0xa8];
    let payload = b"sample";
    assert!(nss::verify_signature(nss::SignatureAlgorithm::ES256, test::NIST_P256_TEST_SPKI,
                                  payload, &signature).is_ok());
}

#[test]
fn test_rfc6979_test_vector_2() {
    test::setup();
    // With SHA-256, message = "test":
    // k = D16B6AE827F17175E040871A1C7EC3500192C4C92677336EC2537ACAEE0008E0
    // r = F1ABB023518351CD71D881567B1EA663ED3EFCF6C5132B354F28D3B0B7D38367
    // s = 019F4113742A2B14BD25926B49C649155F267E60D3814B4C0CC84250E46F0083
    // (the signature is just the bytes of r followed by the bytes of s)
    let signature =
        vec![0xf1, 0xab, 0xb0, 0x23, 0x51, 0x83, 0x51, 0xcd, 0x71, 0xd8, 0x81, 0x56, 0x7b, 0x1e,
             0xa6, 0x63, 0xed, 0x3e, 0xfc, 0xf6, 0xc5, 0x13, 0x2b, 0x35, 0x4f, 0x28, 0xd3, 0xb0,
             0xb7, 0xd3, 0x83, 0x67,
             0x01, 0x9f, 0x41, 0x13, 0x74, 0x2a, 0x2b, 0x14, 0xbd, 0x25, 0x92, 0x6b, 0x49, 0xc6,
             0x49, 0x15, 0x5f, 0x26, 0x7e, 0x60, 0xd3, 0x81, 0x4b, 0x4c, 0x0c, 0xc8, 0x42, 0x50,
             0xe4, 0x6f, 0x00, 0x83];
    let payload = b"test";
    assert!(nss::verify_signature(nss::SignatureAlgorithm::ES256, test::NIST_P256_TEST_SPKI,
                                  payload, &signature).is_ok());
}

#[test]
fn test_tampered_signature_es256() {
    test::setup();
    // Based on test_rfc6979_test_vector_2.
    // With SHA-256, message = "test":
    // k = D16B6AE827F17175E040871A1C7EC3500192C4C92677336EC2537ACAEE0008E0
    // r = F1ABB023518351CD71D881567B1EA663ED3EFCF6C5132B354F28D3B0B7D38367
    // s = 019F4113742A2B14BD25926B49C649156F267E60D3814B4C0CC84250E46F0083
    //                                     ^ this was a 5
    // (the signature is just the bytes of r followed by the bytes of s)
    let signature =
        vec![0xf1, 0xab, 0xb0, 0x23, 0x51, 0x83, 0x51, 0xcd, 0x71, 0xd8, 0x81, 0x56, 0x7b, 0x1e,
             0xa6, 0x63, 0xed, 0x3e, 0xfc, 0xf6, 0xc5, 0x13, 0x2b, 0x35, 0x4f, 0x28, 0xd3, 0xb0,
             0xb7, 0xd3, 0x83, 0x67,
             0x01, 0x9f, 0x41, 0x13, 0x74, 0x2a, 0x2b, 0x14, 0xbd, 0x25, 0x92, 0x6b, 0x49, 0xc6,
             0x49, 0x15, 0x6f, 0x26, 0x7e, 0x60, 0xd3, 0x81, 0x4b, 0x4c, 0x0c, 0xc8, 0x42, 0x50,
             0xe4, 0x6f, 0x00, 0x83];
    let payload = b"test";
    assert!(nss::verify_signature(nss::SignatureAlgorithm::ES256, test::NIST_P256_TEST_SPKI,
                                  payload, &signature).is_err()); // TODO: match specific error
}

#[test]
fn test_tampered_message_es256() {
    test::setup();
    // Based on test_rfc6979_test_vector_2.
    // With SHA-256, message = "test":
    // k = D16B6AE827F17175E040871A1C7EC3500192C4C92677336EC2537ACAEE0008E0
    // r = F1ABB023518351CD71D881567B1EA663ED3EFCF6C5132B354F28D3B0B7D38367
    // s = 019F4113742A2B14BD25926B49C649155F267E60D3814B4C0CC84250E46F0083
    // (the signature is just the bytes of r followed by the bytes of s)
    let signature =
        vec![0xf1, 0xab, 0xb0, 0x23, 0x51, 0x83, 0x51, 0xcd, 0x71, 0xd8, 0x81, 0x56, 0x7b, 0x1e,
             0xa6, 0x63, 0xed, 0x3e, 0xfc, 0xf6, 0xc5, 0x13, 0x2b, 0x35, 0x4f, 0x28, 0xd3, 0xb0,
             0xb7, 0xd3, 0x83, 0x67,
             0x01, 0x9f, 0x41, 0x13, 0x74, 0x2a, 0x2b, 0x14, 0xbd, 0x25, 0x92, 0x6b, 0x49, 0xc6,
             0x49, 0x15, 0x6f, 0x26, 0x7e, 0x60, 0xd3, 0x81, 0x4b, 0x4c, 0x0c, 0xc8, 0x42, 0x50,
             0xe4, 0x6f, 0x00, 0x83];
    let payload = b"testTAMPERED";
    assert!(nss::verify_signature(nss::SignatureAlgorithm::ES256, test::NIST_P256_TEST_SPKI,
                                  payload, &signature).is_err()); // TODO: match specific error
}

// SEQUENCE
//   SEQUENCE
//     OID: 1.2.840.113549.1.1.1 (rsaEncryption)
//     NULL
//  BIT STRING
//    SEQUENCE
//      INTEGER (n)
//      INTEGER (e)
static FIPS_RSA_3072_SPKI: &'static [u8] =
    &[0x30, 0x82, 0x01, 0xA2,
            0x30, 0x0d,
                  0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01,
                  0x05, 0x00,
            0x03, 0x82, 0x01, 0x8f,
                  0x00, 0x30, 0x82, 0x01, 0x8a, // leading 0x00 is "0 unused bits"
                              0x02, 0x82, 0x01, 0x81, 0x00, 0xa7, 0xa1, 0x88, 0x2a, 0x7f, 0xb8,
                                    0x96, 0x78, 0x60, 0x34, 0xd0, 0x7f, 0xb1, 0xb9, 0xf6, 0x32,
                                    0x7c, 0x27, 0xbd, 0xd7, 0xce, 0x6f, 0xe3, 0x9c, 0x28, 0x5a,
                                    0xe3, 0xb6, 0xc3, 0x42, 0x59, 0xad, 0xc0, 0xdc, 0x4f, 0x7b,
                                    0x9c, 0x7d, 0xec, 0x3c, 0xa4, 0xa2, 0x0d, 0x34, 0x07, 0x33,
                                    0x9e, 0xed, 0xd7, 0xa1, 0x2a, 0x42, 0x1d, 0xa1, 0x8f, 0x59,
                                    0x54, 0x67, 0x3c, 0xac, 0x2f, 0xf0, 0x59, 0x15, 0x6e, 0xcc,
                                    0x73, 0xc6, 0x86, 0x1e, 0xc7, 0x61, 0xe6, 0xa0, 0xf2, 0xa5,
                                    0xa0, 0x33, 0xa6, 0x76, 0x8c, 0x6a, 0x42, 0xd8, 0xb4, 0x59,
                                    0xe1, 0xb4, 0x93, 0x23, 0x49, 0xe8, 0x4e, 0xfd, 0x92, 0xdf,
                                    0x59, 0xb4, 0x59, 0x35, 0xf3, 0xd0, 0xe3, 0x08, 0x17, 0xc6,
                                    0x62, 0x01, 0xaa, 0x99, 0xd0, 0x7a, 0xe3, 0x6c, 0x5d, 0x74,
                                    0xf4, 0x08, 0xd6, 0x9c, 0xc0, 0x8f, 0x04, 0x41, 0x51, 0xff,
                                    0x49, 0x60, 0xe5, 0x31, 0x36, 0x0c, 0xb1, 0x90, 0x77, 0x83,
                                    0x3a, 0xdf, 0x7b, 0xce, 0x77, 0xec, 0xfa, 0xa1, 0x33, 0xc0,
                                    0xcc, 0xc6, 0x3c, 0x93, 0xb8, 0x56, 0x81, 0x45, 0x69, 0xe0,
                                    0xb9, 0x88, 0x4e, 0xe5, 0x54, 0x06, 0x1b, 0x9a, 0x20, 0xab,
                                    0x46, 0xc3, 0x82, 0x63, 0xc0, 0x94, 0xda, 0xe7, 0x91, 0xaa,
                                    0x61, 0xa1, 0x7f, 0x8d, 0x16, 0xf0, 0xe8, 0x5b, 0x7e, 0x5c,
                                    0xe3, 0xb0, 0x67, 0xec, 0xe8, 0x9e, 0x20, 0xbc, 0x4e, 0x8f,
                                    0x1a, 0xe8, 0x14, 0xb2, 0x76, 0xd2, 0x34, 0xe0, 0x4f, 0x4e,
                                    0x76, 0x6f, 0x50, 0x1d, 0xa7, 0x4e, 0xa7, 0xe3, 0x81, 0x7c,
                                    0x24, 0xea, 0x35, 0xd0, 0x16, 0x67, 0x6c, 0xec, 0xe6, 0x52,
                                    0xb8, 0x23, 0xb0, 0x51, 0x62, 0x55, 0x73, 0xca, 0x92, 0x75,
                                    0x7f, 0xc7, 0x20, 0xd2, 0x54, 0xec, 0xf1, 0xdc, 0xbb, 0xfd,
                                    0x21, 0xd9, 0x83, 0x07, 0x56, 0x1e, 0xca, 0xab, 0x54, 0x54,
                                    0x80, 0xc7, 0xc5, 0x2a, 0xd7, 0xe9, 0xfa, 0x6b, 0x59, 0x7f,
                                    0x5f, 0xe5, 0x50, 0x55, 0x9c, 0x2f, 0xe9, 0x23, 0x20, 0x5a,
                                    0xc1, 0x76, 0x1a, 0x99, 0x73, 0x7c, 0xa0, 0x2d, 0x7b, 0x19,
                                    0x82, 0x2e, 0x00, 0x8a, 0x89, 0x69, 0x34, 0x9c, 0x87, 0xfb,
                                    0x87, 0x4c, 0x81, 0x62, 0x0e, 0x38, 0xf6, 0x13, 0xc8, 0x52,
                                    0x1f, 0x03, 0x81, 0xfe, 0x5b, 0xa5, 0x5b, 0x74, 0x82, 0x7d,
                                    0xad, 0x3e, 0x1c, 0xf2, 0xaa, 0x29, 0xc6, 0x93, 0x36, 0x29,
                                    0xf2, 0xb2, 0x86, 0xad, 0x11, 0xbe, 0x88, 0xfa, 0x64, 0x36,
                                    0xe7, 0xe3, 0xf6, 0x4a, 0x75, 0xe3, 0x59, 0x52, 0x90, 0xdc,
                                    0x0d, 0x1c, 0xd5, 0xee, 0xe7, 0xaa, 0xac, 0x54, 0x95, 0x9c,
                                    0xc5, 0x3b, 0xd5, 0xa9, 0x34, 0xa3, 0x65, 0xe7, 0x2d, 0xd8,
                                    0x1a, 0x2b, 0xd4, 0xfb, 0x9a, 0x67, 0x82, 0x1b, 0xff, 0xed,
                                    0xf2, 0xef, 0x2b, 0xd9, 0x49, 0x13, 0xde, 0x8b,
                              0x02, 0x03, 0x14, 0x15, 0xa7];


#[test]
fn test_fips186_3_test_vector_1() {
    test::setup();
    let signature = vec![0x43, 0x35, 0x70, 0x7d, 0xa7, 0x35, 0xcf, 0xd1, 0x04, 0x11, 0xc9, 0xc0,
                         0x48, 0xca, 0x9b, 0x60, 0xbb, 0x46, 0xe2, 0xfe, 0x36, 0x1e, 0x51, 0xfb,
                         0xe3, 0x36, 0xf9, 0x50, 0x8d, 0xc9, 0x45, 0xaf, 0xe0, 0x75, 0x50, 0x3d,
                         0x24, 0xf8, 0x36, 0x61, 0x0f, 0x21, 0x78, 0x99, 0x6b, 0x52, 0xc4, 0x11,
                         0x69, 0x30, 0x52, 0xd5, 0xd7, 0xae, 0xd9, 0x76, 0x54, 0xa4, 0x00, 0x74,
                         0xed, 0x20, 0xed, 0x66, 0x89, 0xc0, 0x50, 0x1b, 0x7f, 0xba, 0xc2, 0x1d,
                         0xc4, 0x6b, 0x66, 0x5a, 0xc0, 0x79, 0x76, 0x00, 0x86, 0x41, 0x44, 0x06,
                         0xcd, 0x66, 0xf8, 0x53, 0x7d, 0x1e, 0xbf, 0x0d, 0xce, 0x4c, 0xf0, 0xc9,
                         0x8d, 0x4c, 0x30, 0xc7, 0x1d, 0xa3, 0x59, 0xe9, 0xcd, 0x40, 0x1f, 0xf4,
                         0x97, 0x18, 0xfd, 0xd4, 0xd0, 0xf9, 0x9e, 0xfe, 0x70, 0xad, 0x8d, 0xd8,
                         0xba, 0x13, 0x04, 0xce, 0xfb, 0x88, 0xf2, 0x4b, 0x0e, 0xed, 0xf7, 0x01,
                         0x16, 0xda, 0x15, 0x93, 0x2c, 0x76, 0xf0, 0x06, 0x95, 0x51, 0xa2, 0x45,
                         0xb5, 0xfc, 0x3b, 0x91, 0xec, 0x10, 0x1f, 0x1d, 0x63, 0xb9, 0x85, 0x3b,
                         0x59, 0x8c, 0x6f, 0xa1, 0xc1, 0xac, 0xdb, 0xac, 0xf9, 0x62, 0x63, 0x56,
                         0xc7, 0x60, 0x11, 0x9b, 0xe0, 0x95, 0x56, 0x44, 0x30, 0x18, 0x96, 0xd9,
                         0xd0, 0xd3, 0xea, 0x5e, 0x64, 0x43, 0xcb, 0x72, 0xca, 0x29, 0xf4, 0xd4,
                         0x52, 0x46, 0xd1, 0x6d, 0x74, 0xd0, 0x05, 0x68, 0xc2, 0x19, 0x18, 0x2f,
                         0xeb, 0x19, 0x11, 0x79, 0xe4, 0x59, 0x3d, 0xc1, 0x52, 0xc6, 0x08, 0xfd,
                         0x80, 0x53, 0x63, 0x29, 0xa5, 0x33, 0xb3, 0xa6, 0x31, 0x56, 0x68, 0x14,
                         0xcd, 0x65, 0x4f, 0x58, 0x7c, 0x2d, 0x8c, 0xe6, 0x96, 0x08, 0x5e, 0x6e,
                         0xd1, 0xb0, 0xb0, 0x27, 0x8e, 0x60, 0xa0, 0x49, 0xec, 0x7a, 0x39, 0x9f,
                         0x94, 0xfc, 0xca, 0xe6, 0x46, 0x23, 0x71, 0xa6, 0x96, 0x95, 0xef, 0x52,
                         0x5e, 0x00, 0x93, 0x6f, 0xa7, 0xd9, 0x78, 0x1f, 0x9e, 0xe2, 0x89, 0xd4,
                         0x10, 0x5e, 0xe8, 0x27, 0xa2, 0x79, 0x96, 0x58, 0x30, 0x33, 0xce, 0xdb,
                         0x2f, 0x29, 0x7e, 0x7b, 0x49, 0x26, 0xd9, 0x06, 0xce, 0x0d, 0x09, 0xd8,
                         0x41, 0x28, 0x40, 0x6a, 0xb3, 0x3d, 0x7d, 0xa0, 0xf8, 0xa1, 0xd4, 0xd2,
                         0xf6, 0x66, 0x56, 0x86, 0x86, 0xc3, 0x94, 0xd1, 0x39, 0xb0, 0xe5, 0xe9,
                         0x93, 0x37, 0x75, 0x8d, 0xe8, 0x59, 0x10, 0xa5, 0xfa, 0x25, 0xca, 0x2a,
                         0xa6, 0xd8, 0xfb, 0x1c, 0x77, 0x72, 0x44, 0xe7, 0xd9, 0x8d, 0xe4, 0xc7,
                         0x9b, 0xbd, 0x42, 0x6a, 0x5e, 0x6f, 0x65, 0x7e, 0x37, 0x47, 0x7e, 0x01,
                         0x24, 0x74, 0x32, 0xf8, 0x37, 0x97, 0xfb, 0xf3, 0x1b, 0x50, 0xd0, 0x2b,
                         0x83, 0xf6, 0x9d, 0xed, 0x26, 0xd4, 0x94, 0x5b, 0x2b, 0xc3, 0xf8, 0x6e];
    let payload = vec![0xc1, 0x64, 0x99, 0x11, 0x0e, 0xd5, 0x77, 0x20, 0x2a, 0xed, 0x2d, 0x3e, 0x4d,
                       0x51, 0xde, 0xd6, 0xc6, 0x63, 0x73, 0xfa, 0xef, 0x65, 0x33, 0xa8, 0x60, 0xe1,
                       0x93, 0x4c, 0x63, 0x48, 0x4f, 0x87, 0xa8, 0xd9, 0xb9, 0x2f, 0x3a, 0xc4, 0x51,
                       0x97, 0xb2, 0x90, 0x97, 0x10, 0xab, 0xba, 0x1d, 0xaf, 0x75, 0x9f, 0xe0, 0x51,
                       0x0e, 0x9b, 0xd8, 0xdd, 0x4d, 0x73, 0xce, 0xc9, 0x61, 0xf0, 0x6e, 0xe0, 0x7a,
                       0xcd, 0x9d, 0x42, 0xc6, 0xd4, 0x0d, 0xac, 0x9f, 0x43, 0x0e, 0xf9, 0x03, 0x74,
                       0xa7, 0xe9, 0x44, 0xbd, 0xe5, 0x22, 0x00, 0x96, 0x73, 0x74, 0x54, 0xf9, 0x6b,
                       0x61, 0x4d, 0x0f, 0x6c, 0xdd, 0x9f, 0x08, 0xed, 0x52, 0x9a, 0x4a, 0xd0, 0xe7,
                       0x59, 0xcf, 0x3a, 0x02, 0x3d, 0xc8, 0xa3, 0x0b, 0x9a, 0x87, 0x29, 0x74, 0xaf,
                       0x9b, 0x2a, 0xf6, 0xdc, 0x3d, 0x11, 0x1d, 0x0f, 0xeb, 0x70, 0x06];
    assert!(nss::verify_signature(nss::SignatureAlgorithm::PS256,
                                  FIPS_RSA_3072_SPKI, &payload,
                                  &signature).is_ok());
}