import math

# RICE ENCODER
def rice_encoder(S, M):
    # If sample value is negative, consider it positive
    # and store signs separately
    if S < 0:
        S = -S
        sign = "-"
    else:
        sign = "+"

    # Compute K using equation, M = 2^K
    K = int(math.ceil(math.log(M, 2)))

    # For S, the number to be encoded, find q and r
    quotient = int(math.floor(S / M))
    remainder = S % M

    # Generate codeword
    q_code_str = "1" * quotient + "0"

    # Find the difference
    diff = int(math.pow(2, K) - M)

    # Write out r in binary
    if remainder < diff:
        b = K - 1
        a = "{0:0" + str(b) + "b}"
        r_code_str = a.format(remainder)
    else:
        b = K
        a = "{0:0" + str(b) + "b}"
        r_code_str = a.format(remainder + diff)

    # Encoded string
    code_word = sign + q_code_str + r_code_str

    return code_word

# RICE DECODER
def rice_decoder(S, M):
    # Separate the sign and K-bits binary
    sign = S[0]
    S = S[1:]

    # Compute K using equation, M = 2^K
    K = int(math.ceil(math.log(M, 2)))

    # Separate q and r by splitting at the first 0
    q_and_r_list = S.split('0', 1)

    # Get quotient, q, by counting number of 1s before the first 0
    q = len(q_and_r_list[0])

    # Get remainder, r, by converting the next K or K-1 bits binary to decimal
    r_bits = q_and_r_list[1]
    if len(r_bits) < K:
        r = int(r_bits, 2)
    else:
        r = int(r_bits[:K], 2)
        diff = int(math.pow(2, K) - M)
        if r >= diff:
            r -= diff

    # Compute encoded number using q * M + r
    S = q * M + r

    # Change sign of the decoded bit
    if sign == '-':
        S = -S

    return S

if __name__ == '__main__':
    encoded = rice_encoder(5, 2)
    decoded = rice_decoder(encoded, 2)
    print(f"Encoded: {encoded}")
    print(f"Decoded: {decoded}")
