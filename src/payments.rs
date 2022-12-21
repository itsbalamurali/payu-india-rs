use std::collections::HashMap;

pub enum PaymentMode {
    //Net Banking
    NB,
    // Credit Card
    CC,
    DC,       // Debit Card
    CASH,     // Cash Card or Wallet
    UPI,      // UPI,
    EMI,      // EMI,
    NEFTRTGS, //NEFT/RTGS
}

pub const WALLET_CODES: HashMap<&str, &str> = HashMap::from([

]);



pub const NETBANKING_CODES: HashMap<&str, &str> = HashMap::from([
    ("Airtel Payments Bank", "AIRNB"),
    ("Axis NB", "AXIB"),
    ("AU Small Finance Bank", "AUSFNB"),
    ("Bank Of India", "BOIB"),
    ("Bank Of Maharashtra", "BOMB"),
    ("Bharat Co-Op Bank", "BHNB"),
    ("Canara Bank", "CABB"),
    ("Catholic Syrian Bank", "CSBN"),
    ("Central Bank of India", "CBIB"),
    ("City Union Bank", "CUBB"),
    ("Corporation Bank", "CRBP"),
    ("Cosmos Bank", "CSMSNB"),
    ("Dena Bank", "DENN"),
    ("Deutsche Bank", "DSHB"),
    ("Development Credit Bank", "DCBB"),
    ("Dhanlaxmi Bank", "DLSB"),
    ("HDFC Bank", "HDFB"),
    ("ICICI", "ICIB"),
    ("IDFC", "IDFCNB"),
    ("Indian Bank", "INDB"),
    ("Indian Overseas Bank", "INOB"),
    ("IndusInd Bank", "INIB"),
    ("Industrial Development Bank of India (IDBI)", "IDBB"),
    ("Jammu and Kashmir Bank", "JAKB"),
    ("Janata Sahakari Bank Pune", "JSBNB"),
    ("Karnataka Bank", "KRKB"),
    ("Karur Vysya - Corporate Netbanking", "KRVBC"),
    ("Kotak Mahindra Bank", "162B"),
    ("Lakshmi Vilas Bank - Corporate Netbanking", "LVCB"),
    ("Lakshmi Vilas Bank - Retail Netbanking", "LVRB"),
    ("Nainital Bank", "TBON"),
    ("Oriental Bank of Commerce", "OBCB"),
    ("Punjab And Maharashtra Co-operative Bank Limited", "PMNB"),
    ("Punjab And Sind Bank", "PSBNB"),
    ("Punjab National Bank - Corporate Banking", "CPNB"),
    ("Punjab National Bank - Retail Banking", "PNBB"),
    ("Saraswat bank", "SRSWT"),
    ("SBI Netbanking", "SBIB"),
    ("Shamrao Vithal Co-operative Bank Ltd", "SVCNB"),
    ("Syndicate Bank", "SYNDB"),
    ("Tamilnad Mercantile Bank", "TMBB"),
    ("The Federal Bank", "FEDB"),
    ("The Karur Vysya Bank", "KRVB"),
    ("The South Indian Bank", "SOIB"),
    ("UCO Bank", "UCOB"),
    ("Union Bank - Corporate Netbanking", "UBIBC"),
    ("Union Bank Of India", "UBIB"),
    ("Vijaya Bank", "VJYB"),
]);

use crate::PayuApiClient;

pub struct Payments {
    client: &'static PayuApiClient
}

impl Payments {
    pub fn new(payu_api_client: &PayuApiClient) -> Self {
        Self {
            client: payu_api_client
        }
    }
}