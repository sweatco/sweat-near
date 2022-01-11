use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};

const ICON: &str = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQAAAQABAAD/2wCEAAoHCBUWFRgWFRYYGRgaGBgaHRwaHBwYHBoYGhwcHBgaGhocIS4lHB4rHxoaJjgnKy8xNTU1GiQ7QDs0Py40NTEBDAwMEA8QHxISHzQrJSs0NDY0PTQ0NDQxNDQ9NDU0NDQ2NjQ0NDQ0NDY0NDQ0NDQ0NDQ0NDQ0NDQ0NDQ0NDQ0NP/AABEIAPEA0QMBIgACEQEDEQH/xAAcAAABBQEBAQAAAAAAAAAAAAAAAQMEBQYCBwj/xAA+EAABAwIDBQYEBAUBCQAAAAABAAIRAyEEEjEFQVFhcQYigZGhsTJCwfATctHhBzNSYoLxFBUjNEOSssLD/8QAGgEAAgMBAQAAAAAAAAAAAAAAAAIBAwQFBv/EACoRAAICAQQCAgEDBQEAAAAAAAABAhEDBBIhMUFREyIyYXGRI4Gx4fAU/9oADAMBAAIRAxEAPwD2ZCEIAEIQgAQhCABCEIAYxFXK1x4AnyCyVTtDUpu73eaSCeV5gLT7V/kv/KvPdt03OaA2x3defLeubrckozSi64NenhGSdo9FwmLZUaHscC07/foVJXm3ZLa/4b2gu7jyARuDjYHlfXovSJWrT5vlj+q7KcuPZKvB0hIlWgqBCEIAEIQgBEIUbGYttNsuPQbyeASykoq2Sk26R3VrtbdxAuBcxcmAE6CvOcfiXPe5zjMkmNw3WHQAeCqMdjqtBzKlNxblcNCRO/K4aOaRNj6LAtcnKq4NP/ldd8nryFXbC2kMRQZWy5c02mYhxbEwJ0ViugmmrRlap0KhCFIAhCEACEIQAIQhAAhCEACEIQBX7ZMUXcwB6heddpc2Tu248SJ0HjC9B2+6KJHFzR6z9Fg8TUucxjd9hcjXS/qr9jdpl9f7md2XWtl4XHOSV6x2Z2iK1Ft+8wBrhzAsfEfVeRPwtRji9ozDMYIE68tQtDsDadSk7M1jhuc11gQkw5finfh9lmWG+NeUeqIVJh+0tBwGbM07wWl3q2VJbt3Dn/qDxBHuF1VmxvpowPHJeGWaExh8Q14zNIIkixm4sdE6Hg6EKxNPlC0dpEJutVa1pc4gNAkk7ghuiDjFYlrGy4wPUngFjdrbRL3E79AOA/VJtfaxeSRZt8oO4cTzKpmvtLjrfzXH1WpeR7Y9f5N+HDtVvs6ncqzb7jkAGhdf6ffNPPrHNITlQh7CC0GwseO9ZIunZoaNN/DPGl9B9Mx/w3W4w+XX8Z8ltF5n/DGuWYitRI+Jgd0NN2X1z+i9MXd07vGjm5lU2KhCFeVAhCEACEIQAIQhAAhCEAIhcPeAJJAHMwqfaG2QBlp3J+bcOnEqrJljjVyY0YSk6SIPaXaQJFNty0yeE7h11Wdo4Mudnf1A/VTmUgDO/ibrpcPLkeSbkzp44KEdqOPwW7gFAqsymNVYPdAJ4KrxNfedTuVYwqEBCAFbUcAQCQDqASAeq6pVnN+Fzm/lJbppouEKdzIomDbFZg/mujmc3vKbxWPqPEPeXAXgmB1jRRntBBBEgqrx9J5hmaWS2bd6J0J3pvkm+HJ1+5ChHwh0VTVdDZ/DGp/rPAf2+6dx+KaxtxPAJzCshqrdrMYCSRLuX1P30Sqmxhpu0WHWRyj9FY4CoHCRvAN/bzWYiTYfVaPZtLI2/wAoN08opdEp2WHYam5uPcGkQKTsxI+ISySOHeLfAHivUV5p/DWmX4mvU4My/wDe4H/5r0sLsaRVjRzdQ/uKhCFpKQQhCABCEIAEIQgBIVftPaIpiBdx0HDmeSm1agAJOgBPgNVjcXXL3Ocd58huHksmqzvHGo9suw497t9IStWLjmc4k893Qbk0hR8Zj6VETVqNYDpncGz0nVcfmT9s38JEhI0g3F1SYrajajR+G4Fh+YEEO8lN2U+GEnTMfooGriyRjHDLHRVD+8+NwUrE1NSbj6KPhm2niVAIfXRpu4HyUrC0RAcdVJQBVIUnFsAMiL7vqoyABI4SISoQAxnDGS4gATc/d1msZii9x4e/Vaeth2P+NoMcVHqbOYXNdlADZsBAJ3Sni0uwKnZeDzEOOm79VcYupkYcocbG4gRzk7/NPsYGjzSMMmZEDQfUqHK3bA0H8MqTRh3uB7zqpzcQABlHPUnxK2qwfY6s2i/Jo19v8p7vnceIW8ldrSzUsarwc3MmpsVCELSVAhCEACEIQAIQhAFZtytFIjeSAPOT6BVuzcCCMzryOGkqXj6GeoJPdbaOe8z96KUxoAgLnZI78rk+lwjTF7YUvJWbQw7crjYReTuAufReQdssPTNWnUxTn5aoeWCmMxZRY3uw1xDS973CSbBoO+I9T284va+mPmbYzoSCLrzPtfSGIoUYtXw4LHMdbOwxdp0zAtBiRYngJbT7N7flEZNyil4Mz2dqij+HUbVaXVKppGgJLssCKpOg7zgADc3jfHp7GZWht+J6nl0hZLsL2Uc14xNYQxg7oN+9/V4cQtpkL3dwH74rNrXGU/r35L9PuUeRkhLkiByEdNyn4XAmZduOnG0/oo21cQ3PHC3Of0sse2lbL7t0iawCBGiUlUmM2/To0y55gAeJO4AKjirjKL8Viq7sHgwQGZWlz35nBocQL5ZI01vuEq/DppZXx17K8mSMOzVYq5zC40kGbqOvJ62MrYSu9lHFCqGujNTcXU6ggEGHWOsHgQYO9b/s5t9mKZoG1GgZm7vzN/tPomz6SWJbu0Rjzxk68lyhCFlLgQiUIACoVmnn6BTVHxNIRI1QSh6gSIg3BkHQ8ivQtjY38ak1x10PUakcivPWCAFrux9eWPbwcD4OEf8Ar6rboZuOTb7M2pjcb9GlQhC7BgBCEIAEIQgBFzPFLKYxLxlPl5qJOlZKVkWm2SXHinVwwwBwjy6romFiLjObRBzmdN3Rcf7vpWc9jXPje0HKN3il2l2jw7HFmfM4EzlBdB6i0+KgU+0OHcfjI/M13vCyvFOMnJJl6mnFJk3FU3OytbYfMeQ3ffBOUqLWjuj9fNdMeHAOaQQdCDIPiF0qq5Hs4rPIBI3X8N6z1ahnk8N+tjxWie2QQd4IUfDYXKwtOrtY9kko2xoujyXtrs1+dhc+GOD4J0DhcZuEzErV7be3FbGacOLUzTeWDVraYLHtjflmeYbKt8QxslpAIki4mR0VeNkUQSWsDCTJyd2TxgWnwWvDrFCKjJdeinJhcm5J9nkdPCPe6GtLiTu+p3Bb/sv2cFMtrEmQ0iZIDiRBgf09dYlaBmzWSLF3AG4noNVfbL2U6o7vAtYNTp0aOd1OXVPKtsF2RDCoPdJjuwNnh7nOe2WgWmR3jEdbT5hXr9kUDcsaLRqR9deeqmUqYa0NaIAAA6CyDSBMkSVMMSjGqTFlkcnZDr7MY5oYJa0fKzKAetpKgu2DkeHMyubva/6ED3V6hM8UX2hVOSKbG7JDg4tZENIaBF3E3N+VpJ8LLL1aZBLSNCQeo1C9BWQ2/ScKjnOu35bRbhH13rPqMaS3IuwzbdMqwrvsm8isQNCwz4EQRz/UqkVt2YIFdsjUOjkY18pHiqNO6yr9y3L+DN2hIlXoDmAhCEACSEFcX4epQAuX7kqBtB4s3MON48PqpuU8vb2VXi/iPw2gaF31VOZ1EaK5G21iLAg+v7+6y/8AFLatXC4CaJIdUqNpl41a0tc5+U6Xyhs/3HSFp6A74BjXg0ehJKpv4jVGNwFTNTbUu05HQ2Qx0vII0IbJEXCrwRVWxpPwfPVTbFcxNR1uFvbVSWdo8QBGZp5lon0sq/GVGOeTTYWN/pLs8H80CyMPRa+xe1h3ZpDT/kJg9R4rSIT9k9ocThnl9Ko4EmXAnM1/HM02PCdeBC+guytb/bMMyvGUuaMzODjwdvEXHVeA7D7PvxFf8OYY0jO8EOa1sTYixJAsvo3sthxSpZGgANytGpEDWBoddR+5rnihN8oaM5RXDErYaLi0biojpmIhXGLbmhwBnXvQ2ANbGIjjc3VZizMGR0zN0WDNh2M0QyX2cYbZNOoHF2YGdQdJA+oPmnj2epRYunrrbfbjeyk7MPdM6br+BU8BEcUHFNoiU5J8Mq8FsRtNzXZnFw4xE9IsrQCNEqFbGMYqoiNuXYIQhMQCEIQALLdpabg8OM5CBF7TvgT0WpWX7SUH5w8juwGgyD6QCPXqs+o/AsxfmUifwVc03tePldPUbx4gkeKaDV1C52/a7Rt22uT0im8OaHC4IBHQ3C7VT2aq5qAB+Ulv1HoQrdeixz3wUvaOVJVJoEJUJxQSSgohSByfP2VXiW94yd/QdIVqSqrHA5jzA++Sozr6jw7GBVykHSD99PUrO9rqzi+HfA5stmDDYE/FzlaJrI013lV+38Kx1E5rQZBiZJtB5FUQnt4LHGzxLb/ZRweXUACCSSyzY/LJuNSdI9mdl9k3k5q/daC2zS1xcCbyQYDY3idRCvsZ2rZTcQ6hUifilpDoPdIvA0BjVOYftDhK4LTDXXOWoAAbgmHXbeBabrVcqKqRebFwLKYYWM+DK3LkPddlJLiTcgggZr716PgHsbTaWhrQZeXEgNdJiS6LuI0B4cl55hg2XklzmvDYIdLRlkENA9brXbL2icgz0RJAa6wGZrfhDh82tjG5J8kY8yZO1vovnVaTZaRlygWgEAO3C19NAs85wJPeEyeXlyT2KxGcCBlGpEkgGI7s/CIiw6pkDj63Cw6jMptJdI0Y4bey2wzSWNIMQbidb85Uj8WDGXxF7cVW0e7GW3Q2UulXOWJiOWoTRnwLJE1pXSh0sUJvafEdeWgTlTFAAEXn7unUlVi7WSExUxTRzPJQqtYuMzHAcE2klk9DKPslnGngFyMY7lr9hRS4JGkpN79jbUSnY119PJUW2Md+JlbuaTfjIEH3Vo8iLrNLPnySrbfZdhgrsEISSshoNN2Rq2e3mHDxEH2C0qxXZ+qG1M3gfyn97+C2y7mhnuxJejm6iNTsEJULaUAhCEAcwoGObcHiI+/NWBTOKZLTyv5JMi3RaGi6ZWAKNjqGdjm8vZSSVHe+BefvT/Rc6T4L0eZbV7OvY85ILSJgm5M6Dl1Wbxex2zlfSAceUE3j5dbr1+tSa/4h+3RRhs1kzc8v3RDVNLkZ40zyHC4F9Hv0Kr6ea25zT4EQdN61uze2dVkNxFIOA1fSuY0uwmedpW0dg2FmUsbl0iN2igP7N4cmcg1BjTTpCaWohP8AJEKDj0yixXbku/5XD1arT8xa4MneBMIwm3do1hDcM2nNsznGwO/KBJjhI6rX08KxujR7+Up6FT8kEvrH+RtsvLKjYeExDHOfXrF8gANyhjW33AEnzK0LDIUYBSGNgJVJydktJHSELgvCZuiBXVAE0ahXCcZTlV7m+hqSOWsJT7WwkY2EPeACToBKaKohuyq2lWcHubuIbHSZPqFAUzadYOfA3WJ4n9rqESseR3JmmCqKAlM06uZxjQDXmf8ARRa1cm026QptCmGhJVFhNwNWHRx+wtxs2pmptPKPKyweGfldMSYsOZ+ythsJ3dd1HqF0NBOpV7MeqjxZboQhdkwghCEACEIQBmtoNNJ5v3XCR53Cj1Hyd/ints1Mzz/bYdRqqtuKh2R28SDxjUHmuHmklNpdWb4Rbin5JaEiVIAiVCEAEJ1lPihlQQnQnikK2xGtASoJTNV+5O2kgSs6dUTTnSkQqnJslKgT1Ib0jafFOTuTRjXLIbFVbtasRDBoRJ87BWD3ECwk9YVbjDlOd+Uv+VuoaOJ4oyv60NjX2srVDxdb5RHNdY6s7jrqZuVBWRLya0ScIxpN9dwU9Q8FTOsKa1pJAGpSy7BknA0pdm3D3Wn2C67hyafKZ9wqXDU8rQPHxKtNjfzP8T9LHwnyW7SfWaMed7kzQoSJV2zCCEIQAiaxE5XZdcpjrFk6mcTUytceAKWX4sldmVKr8VQztLd+7qrBNVmb1wJo6MHTKzCY11MhlTTc6d272Vu1wOigYmgHtynz4HiqxzKtIgNJI3QJHMRuKRSoscVI0iFRN22Yuy/W3snxttsDumd+ifcit45FslDiqaptwSMrJG+THkuTtzgz1U7kHxsuiULPu2086BoHiVxRxVd5hrjbfYD2uo3In42aCpUa0S4gD704pMNVcZJAHAawOZ4qDQwwBzOJc7eTBI6GLBWlJnK3kiLbfBEkoo7BzarpjblJnaNF3KvVFLFUHF4QvNrDUnid3MnnuU5QdoYvKMjfiI8gfqlybdvI0LvgoKtETmO7XmoLGFxtvUusS85G+J+iscBs3Ld32f0WWKbNbkorkaoskgD0VrTw7W6BFHDtboL8U6njGuyiU76BS+y1SXVAdRlPuCq3GVIYeJsu+zFQivE/E1084v5/ur8MtuaP/diTjeNs2iEIXcMAIQhACKDtafwzHj04ecKcq3bVWGRvcfQXP0VWZpY3foaH5IokhSoXFNpHfTjom1LKYqU46KuUaLYyvhlBtKjlfPH1/f76Qlpa1IPaWnQ/cqvxWzu4C2JbrukfqlLUyqXbGEkAakwuQFZYDDd8zBGUE7xJ0H1QS2JhtnE/GCOcj2+9FaUaQaA1oTrGSn2sAUqNlcpUctpDenEIViVFLbYIlCFJB018KK7CgkkkyZv1/ZSEAKGr4ZKddDdLDNEBrQI0UltPiu2sAXSsjBIRybOPwwmXiCpKYqm6mSVBFlftE2HX/Rd9nf57P8v/ABKa2i24PGVzsp+WtTP97R4EwfdZ4OssW/aL6vG1+jPQEIQvQ2csVCEJgEWe2riMzoGjbeO8/fBaFZrH4NzHTq0mx+h5rHrN2zjryW4a3ckVCELmGoEhCVCAIz2QmyJsVJrCyjqqSpl0XaKChhi5+QHQmTyCvg0DRDWAEkC51PFPUmzqFCVkykOtaAukIVxSCEIQQCF2KZToYOCZRbIbGWMJTrGQu0J1FIhuwQhCYgCoOJqgAmeMJ/F1Q1t/vgqSrVLtVnzTrgtxxvk4c8nVOYMS9g4vYPNwTStdgbPNSoHaNY4Eni4GQ0fVU4oynkSRbOSjFtmzQu4QvQ7GcqxUIQrSBFxUphwIIkHcnEijsDPY/Z5Z3m3b6jry5qCteqnG7KnvMseG7w4Ln59K19ofwaIZfEimQlc0gwQQRuKRYS85dpZRVMXApiZSyjYylQyxklSGiEAJQiMaCUrBACXKnWU4TxjYjYjKfFOBoSoVqSQtghCFIAhCEACbr1msEuP7pxU+PqF57t2jfG/f1STltQ0Y7mc4vHZxlAgTPNQ10WHNlg5piNTPKNVf7L7PEw6tYbmjU/mI06D0WeGPJmlSX+i6U4Y48ldsrZTqzp0YDc/RvE+y2uGoNY0NaIA0C6p0w0ANAAFgBYALsLtafTRwrjl+zBlyub/QVKkQtJUKhCEACEIQAIQhAEfE4ZrxDh0O8dCqfE7Kc27e8PXy3q/QVTkwQn2uR4zcejIOEWNihamrh2u+IA+/nqodTZY+VxHI3WKejkvxdlyzJ9lIGHgu6TIU9+z3jcD0P6wmHUXDVrh4FVfDKL5TG3pjcJUFCCQQhCABCF01hOgJ6CUAcoT7cI86NPjb3Uinsx29wHS6sWKb6QrlFeSAU9htnk6NDRxiPIb1bUcG1twJPE3P7eCkq+Gl8yK5ZfRGw+Da24EnidVJQlWuMVFUiptvsEIQmIBCEIAEIQgAQhCABCEIARCEIAVIlQgBEIQoZI1V0VXitClQseYtgVrdVa0NAhCz4uyyZYU04hC3R6M7FSoQrkKCEIUgIlQhAAhCEACEIQB//9k=";
const DECIMALS: f64 = 1000000000000000000.;
const K: f64 = 0.9999999999999762;
const DAILY_STEP_CONVERSION_LIMIT: u32 = 10_000;
const DAY_IN_NANOS: u64 = 86_400_000_000_000;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    oracles: LookupSet<AccountId>,
    token: FungibleToken,
    steps_from_tge: U64,
    daily_limits: LookupMap<AccountId, (u32, u64)>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(oracles_vec: Vec<AccountId>) -> Self {
        let mut oracles_tree = LookupSet::new(b"s");
        for oracle in oracles_vec.iter() {
            env::log_str(oracle.as_str());
            oracles_tree.insert(oracle);
        }
        Self {
            oracles: oracles_tree,
            token: FungibleToken::new(b"t"),
            steps_from_tge: U64::from(0),
            daily_limits: LookupMap::new(b"l"),
        }
    }

    pub fn get_steps_from_tge(&self) -> U64 {
        self.steps_from_tge
    }

    pub fn record_batch(&mut self, steps_batch: Vec<(AccountId, u32)>) {
        assert!(self.oracles.contains(&env::predecessor_account_id()));
        let mut oracle_fee = 0f64;
        for (account_id, steps) in steps_batch.into_iter() {
            if !self.token.accounts.contains_key(&account_id) {
                self.token.internal_register_account(&account_id);
            }
            let capped_steps = self.get_capped_steps(&account_id, steps);
            let swt_to_mint = self.formula(self.steps_from_tge, capped_steps);
            let trx_oracle_fee = swt_to_mint.0 as f64 * 0.05;
            let minted_to_user = swt_to_mint.0 as f64 - trx_oracle_fee;
            oracle_fee = oracle_fee + trx_oracle_fee;
            self.token.internal_deposit(&account_id, minted_to_user as u128);
            self.steps_from_tge.0 += capped_steps as u64;
        }
        self.token.internal_deposit(&env::predecessor_account_id(), oracle_fee as u128);
    }

    pub fn formula(&self, steps_from_tge: U64, steps: u32) -> U128 {
        let a: f64 = DECIMALS * (K.powf(steps as f64 + steps_from_tge.0 as f64 + 1.));
        let b: f64 = DECIMALS * (K.powf(steps_from_tge.0 as f64 + 1.));
        let swt: f64 = (a - b) / (K - 1.) / 1000.;
        U128(swt as u128)
    }

    fn get_capped_steps(&mut self, account_id: &AccountId, steps_to_convert: u32) -> u32 {
        let (mut sum, mut ts) = self.daily_limits.get(account_id).unwrap_or((0, 0));
        let current_ts: u64 = env::block_timestamp();
        //let current_ts:u64 = env::block_timestamp() + 10; // :todo: for debug tests
        let mut remaining_steps = 2 * DAILY_STEP_CONVERSION_LIMIT;
        if ts == 0 || current_ts - ts >= DAY_IN_NANOS {
            ts = current_ts;
            sum = 0;
        }

        // TODO can either variable cross u32 bounds? Cast will overflow
        remaining_steps = i32::max(0, remaining_steps as i32 - sum as i32) as u32;
        let capped_steps: u32 = u32::min(remaining_steps, steps_to_convert);
        self.daily_limits
            .insert(account_id, &(sum + capped_steps, ts));
        // println!("time = {}, remaining_steps = {}, steps_to_convert = {}, sum = {}", current_ts, remaining_steps, steps_to_convert, sum);
        capped_steps
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: "ft-1.0".to_string(),
            name: "SWT (v0.3)".to_string(),
            symbol: "SWT (v0.3)".to_string(),
            icon: Some(String::from(ICON)),
            reference: None,
            reference_hash: None,
            decimals: 18,
        }
    }
}

// :TODO: sandbox tests?
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formula() {
        let oracles = vec!["intmainreturn0.testnet".parse().unwrap()];
        let contract = Contract::new(oracles);
        assert_eq!(U64(0), contract.get_steps_from_tge());
        
        // todo - 0 ? 
        let steps_from_tge = vec!(1, 10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000u64, 100_000_000_000u64, 1_000_000_000_000u64, 10_000_000_000_000u64, 100_000_000_000_000u64, 1_000_000_000_000_000u64);
        let steps_to_convert = vec!(1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000); //, 10_000_000_000u64, 100_000_000_000u64, 1_000_000_000_000u64, 10_000_000_000_000u64, 100_000_000_000_000u64, 1_000_000_000_000_000u64);
        let res = [
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998813084112, 99999.88120560747, 999988.1207056076, 9998812.155439252, 99881300.16038784, 988214137.35593, 8900808550.457266, 38178195671.24964, 42089716141.62507], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998813084112, 99999.88120560747, 999988.1207056076, 9998812.15543458,  99881300.1603645, 988214137.3557196, 8900808550.455366, 38178195671.24148, 42089716141.61607], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998813084112, 99999.88120560747, 999988.1207056076, 9998812.155415887, 99881300.16015421, 988214137.3536028, 8900808550.436333, 38178195671.15984, 42089716141.52607], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998813084112, 99999.88120560747, 999988.120682243,  9998812.155200936, 99881300.1580187, 988214137.332472, 8900808550.24601, 38178195670.34348, 42089716140.62607], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998808411214, 99999.88118224299, 999988.1204719626, 9998812.153060747, 99881300.13665888, 988214137.1211636, 8900808548.342758, 38178195662.17988, 42089716131.62607], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999859813084, 9999.998789719626, 99999.88096728972, 999988.1183317757, 9998812.131682243, 99881299.92308411, 988214135.0080748, 8900808529.310251, 38178195580.543846, 42089716041.62607], 
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 999.9999672897196, 9999.998574766354,  99999.8788317757, 999988.0969532711, 9998811.917878503, 99881297.78733644, 988214113.877201, 8900808338.985195, 38178194764.18349, 42089715141.62608], 
            [0.001, 0.01, 0.1, 1.0, 9.999995327102804, 99.99997663551402, 999.9997476635514, 9999.99643457944,  99999.85744859814, 999987.8831214954, 9998809.779841121, 99881276.42982243, 988213902.56843, 8900806435.734856, 38178186600.58088, 42089706141.62726], 
            [0.001, 0.01, 0.1, 1.0, 9.999976635514018, 99.99976168224299, 999.9976121495328, 9999.975056074767,  99999.64362149533, 999985.7448598131, 9998788.39951402, 99881062.85496262, 988211789.4832337, 8900787403.253859, 38178104964.6509, 42089616141.744865], 
            [0.001, 0.01, 0.1, 0.9999766355140186, 9.999766355140187, 99.9976261682243, 999.9762336448599, 9999.761228971962,  99997.50535981308, 999964.3625, 9998574.59875701, 99878927.13147198, 988190658.879771, 8900597080.682188, 38177288614.95178, 42088716153.50542], 
            [0.001, 0.009995327102803738, 0.0999766355140187, 0.9997616822429907, 9.997621495327103, 99.97624299065421, 999.7624252336449, 9997.623214953272,  99976.12528037383, 999750.5640233646, 9996436.842565421, 99857572.40811215, 987979377.6941122, 8898694078.779318, 38169126077.96445, 42079717329.47111], 
            [0.001, 0.009976635514018693, 0.09976635514018692, 0.9976308411214954, 9.976271028037383, 99.76269626168224, 997.6269345794392, 9976.268247663553,  99762.57578037382, 997615.0917897195, 9975084.40316355, 99644276.13087383, 985869048.7709159, 8879686423.440416, 38087596632.55882, 41989834841.47048], 
            [0.0009766355140186915, 0.009766355140186915, 0.09764953271028037, 0.976518691588785, 9.765210280373832, 97.6521214953271, 976.5212336448599, 9765.211289719626,  97652.00848598132, 976509.6445233645, 9764052.493598131, 97536211.57535046, 965012099.6435281, 8691828646.353561, 37281819165.11522, 41101502004.317604], 
            [0.0007850467289719626, 0.007883177570093458, 0.07885046728971963, 0.7885280373831776, 7.88527570093458, 78.85277102803738, 788.5277009345795, 7885.276186915888,  78852.67758411216, 788518.345411215, 7884340.4772757, 78759173.14332244, 779234233.2520328, 7018534206.256364, 30104565302.446426, 33188907591.59628], 
            [9.287383177570093e-05, 0.000929322429906542, 0.00929322429906542, 0.09293282710280375, 0.9293294392523365, 9.293292640186916, 92.93292523364485, 929.3291489485982,  9293.281552570093, 92931.82194334111, 929218.8693358646, 9282261.468661798, 91837631.220618, 827178181.6533574, 3548011430.099568, 3911520472.2099967], 
            [4.8143719540578184e-14, 4.806212001593313e-13, 4.80512400793138e-12, 4.8050424084067345e-11, 4.805031528470115e-10, 4.805030168478037e-09, 4.805030005278988e-08, 4.805029477602062e-07,  4.805024340367989e-06, 4.804972967972857e-05, 0.0004804459285441455, 0.004799326485362628, 0.04748398624172937, 0.4276865253931822, 1.8344737739111292, 2.022423507858977]
        ];

        for tge_index in 0..steps_from_tge.len() {
            for steps_to_convert_index in 0..steps_to_convert.len() {
                let formula_res = contract.formula(U64(steps_from_tge[tge_index]), steps_to_convert[steps_to_convert_index]).0 as f64 / DECIMALS;
                let diff = (formula_res - res[tge_index][steps_to_convert_index]).abs();
                // println!("{}, {}, {}, {}", steps_from_tge[tge_index], steps_to_convert[steps_to_convert_index], formula_res, formula_res - res[tge_index][steps_to_convert_index]);
                assert_eq!(true, diff < 0.000005);
            }
        }
    }

    // #[test]
    // fn test_oracle_fee() {
    //     let oracles = vec!["bob.near".parse().unwrap(), "alice.near".parse().unwrap()];
    //     let mut contract = Contract::new(oracles);
    //     assert_eq!(U64(0), contract.get_steps_from_tge());
        
    //     contract.record_batch(vec!(("alice.near".parse().unwrap(), 1_000_000_000)));
        
    //     // let bob = contract.token.ft_balance_of("bob.near".parse().unwrap());
    //     // let alice = contract.token.ft_balance_of("alice.near".parse().unwrap());

    // }
}
