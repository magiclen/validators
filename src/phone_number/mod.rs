#![cfg(feature = "phone_number")]
extern crate regex;
extern crate phonenumber;

use super::{Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::ops::Deref;
use std::hash::{Hasher, Hash};

use self::phonenumber::country;

lazy_static! {
    static ref COUNTRIES: Vec<country::Id> = {
        let mut countries: Vec<country::Id> = Vec::new();

        if cfg!(feature = "phone_ac") {
            countries.push(country::AC);
        }
        if cfg!(feature = "phone_ad") {
            countries.push(country::AD);
        }
        if cfg!(feature = "phone_ae") {
            countries.push(country::AE);
        }
        if cfg!(feature = "phone_af") {
            countries.push(country::AF);
        }
        if cfg!(feature = "phone_ag") {
            countries.push(country::AG);
        }
        if cfg!(feature = "phone_ai") {
            countries.push(country::AI);
        }
        if cfg!(feature = "phone_al") {
            countries.push(country::AL);
        }
        if cfg!(feature = "phone_am") {
            countries.push(country::AM);
        }
        if cfg!(feature = "phone_ao") {
            countries.push(country::AO);
        }
        if cfg!(feature = "phone_ar") {
            countries.push(country::AR);
        }
        if cfg!(feature = "phone_as") {
            countries.push(country::AS);
        }
        if cfg!(feature = "phone_at") {
            countries.push(country::AT);
        }
        if cfg!(feature = "phone_au") {
            countries.push(country::AU);
        }
        if cfg!(feature = "phone_aw") {
            countries.push(country::AW);
        }
        if cfg!(feature = "phone_ax") {
            countries.push(country::AX);
        }
        if cfg!(feature = "phone_az") {
            countries.push(country::AZ);
        }
        if cfg!(feature = "phone_ba") {
            countries.push(country::BA);
        }
        if cfg!(feature = "phone_bb") {
            countries.push(country::BB);
        }
        if cfg!(feature = "phone_bd") {
            countries.push(country::BD);
        }
        if cfg!(feature = "phone_be") {
            countries.push(country::BE);
        }
        if cfg!(feature = "phone_bf") {
            countries.push(country::BF);
        }
        if cfg!(feature = "phone_bg") {
            countries.push(country::BG);
        }
        if cfg!(feature = "phone_bh") {
            countries.push(country::BH);
        }
        if cfg!(feature = "phone_bi") {
            countries.push(country::BI);
        }
        if cfg!(feature = "phone_bj") {
            countries.push(country::BJ);
        }
        if cfg!(feature = "phone_bl") {
            countries.push(country::BL);
        }
        if cfg!(feature = "phone_bm") {
            countries.push(country::BM);
        }
        if cfg!(feature = "phone_bn") {
            countries.push(country::BN);
        }
        if cfg!(feature = "phone_bo") {
            countries.push(country::BO);
        }
        if cfg!(feature = "phone_bq") {
            countries.push(country::BQ);
        }
        if cfg!(feature = "phone_br") {
            countries.push(country::BR);
        }
        if cfg!(feature = "phone_bs") {
            countries.push(country::BS);
        }
        if cfg!(feature = "phone_bt") {
            countries.push(country::BT);
        }
        if cfg!(feature = "phone_bw") {
            countries.push(country::BW);
        }
        if cfg!(feature = "phone_by") {
            countries.push(country::BY);
        }
        if cfg!(feature = "phone_bz") {
            countries.push(country::BZ);
        }
        if cfg!(feature = "phone_ca") {
            countries.push(country::CA);
        }
        if cfg!(feature = "phone_cc") {
            countries.push(country::CC);
        }
        if cfg!(feature = "phone_cd") {
            countries.push(country::CD);
        }
        if cfg!(feature = "phone_cf") {
            countries.push(country::CF);
        }
        if cfg!(feature = "phone_cg") {
            countries.push(country::CG);
        }
        if cfg!(feature = "phone_ch") {
            countries.push(country::CH);
        }
        if cfg!(feature = "phone_ci") {
            countries.push(country::CI);
        }
        if cfg!(feature = "phone_ck") {
            countries.push(country::CK);
        }
        if cfg!(feature = "phone_cl") {
            countries.push(country::CL);
        }
        if cfg!(feature = "phone_cm") {
            countries.push(country::CM);
        }
        if cfg!(feature = "phone_cn") {
            countries.push(country::CN);
        }
        if cfg!(feature = "phone_co") {
            countries.push(country::CO);
        }
        if cfg!(feature = "phone_cr") {
            countries.push(country::CR);
        }
        if cfg!(feature = "phone_cu") {
            countries.push(country::CU);
        }
        if cfg!(feature = "phone_cv") {
            countries.push(country::CV);
        }
        if cfg!(feature = "phone_cw") {
            countries.push(country::CW);
        }
        if cfg!(feature = "phone_cx") {
            countries.push(country::CX);
        }
        if cfg!(feature = "phone_cy") {
            countries.push(country::CY);
        }
        if cfg!(feature = "phone_cz") {
            countries.push(country::CZ);
        }
        if cfg!(feature = "phone_de") {
            countries.push(country::DE);
        }
        if cfg!(feature = "phone_dj") {
            countries.push(country::DJ);
        }
        if cfg!(feature = "phone_dk") {
            countries.push(country::DK);
        }
        if cfg!(feature = "phone_dm") {
            countries.push(country::DM);
        }
        if cfg!(feature = "phone_do") {
            countries.push(country::DO);
        }
        if cfg!(feature = "phone_dz") {
            countries.push(country::DZ);
        }
        if cfg!(feature = "phone_ec") {
            countries.push(country::EC);
        }
        if cfg!(feature = "phone_ee") {
            countries.push(country::EE);
        }
        if cfg!(feature = "phone_eg") {
            countries.push(country::EG);
        }
        if cfg!(feature = "phone_eh") {
            countries.push(country::EH);
        }
        if cfg!(feature = "phone_er") {
            countries.push(country::ER);
        }
        if cfg!(feature = "phone_es") {
            countries.push(country::ES);
        }
        if cfg!(feature = "phone_et") {
            countries.push(country::ET);
        }
        if cfg!(feature = "phone_fi") {
            countries.push(country::FI);
        }
        if cfg!(feature = "phone_fj") {
            countries.push(country::FJ);
        }
        if cfg!(feature = "phone_fk") {
            countries.push(country::FK);
        }
        if cfg!(feature = "phone_fm") {
            countries.push(country::FM);
        }
        if cfg!(feature = "phone_fo") {
            countries.push(country::FO);
        }
        if cfg!(feature = "phone_fr") {
            countries.push(country::FR);
        }
        if cfg!(feature = "phone_ga") {
            countries.push(country::GA);
        }
        if cfg!(feature = "phone_gb") {
            countries.push(country::GB);
        }
        if cfg!(feature = "phone_gd") {
            countries.push(country::GD);
        }
        if cfg!(feature = "phone_ge") {
            countries.push(country::GE);
        }
        if cfg!(feature = "phone_gf") {
            countries.push(country::GF);
        }
        if cfg!(feature = "phone_gg") {
            countries.push(country::GG);
        }
        if cfg!(feature = "phone_gh") {
            countries.push(country::GH);
        }
        if cfg!(feature = "phone_gi") {
            countries.push(country::GI);
        }
        if cfg!(feature = "phone_gl") {
            countries.push(country::GL);
        }
        if cfg!(feature = "phone_gm") {
            countries.push(country::GM);
        }
        if cfg!(feature = "phone_gn") {
            countries.push(country::GN);
        }
        if cfg!(feature = "phone_gp") {
            countries.push(country::GP);
        }
        if cfg!(feature = "phone_gq") {
            countries.push(country::GQ);
        }
        if cfg!(feature = "phone_gr") {
            countries.push(country::GR);
        }
        if cfg!(feature = "phone_gt") {
            countries.push(country::GT);
        }
        if cfg!(feature = "phone_gu") {
            countries.push(country::GU);
        }
        if cfg!(feature = "phone_gw") {
            countries.push(country::GW);
        }
        if cfg!(feature = "phone_gy") {
            countries.push(country::GY);
        }
        if cfg!(feature = "phone_hk") {
            countries.push(country::HK);
        }
        if cfg!(feature = "phone_hn") {
            countries.push(country::HN);
        }
        if cfg!(feature = "phone_hr") {
            countries.push(country::HR);
        }
        if cfg!(feature = "phone_ht") {
            countries.push(country::HT);
        }
        if cfg!(feature = "phone_hu") {
            countries.push(country::HU);
        }
        if cfg!(feature = "phone_id") {
            countries.push(country::ID);
        }
        if cfg!(feature = "phone_ie") {
            countries.push(country::IE);
        }
        if cfg!(feature = "phone_il") {
            countries.push(country::IL);
        }
        if cfg!(feature = "phone_im") {
            countries.push(country::IM);
        }
        if cfg!(feature = "phone_in") {
            countries.push(country::IN);
        }
        if cfg!(feature = "phone_io") {
            countries.push(country::IO);
        }
        if cfg!(feature = "phone_iq") {
            countries.push(country::IQ);
        }
        if cfg!(feature = "phone_ir") {
            countries.push(country::IR);
        }
        if cfg!(feature = "phone_is") {
            countries.push(country::IS);
        }
        if cfg!(feature = "phone_it") {
            countries.push(country::IT);
        }
        if cfg!(feature = "phone_je") {
            countries.push(country::JE);
        }
        if cfg!(feature = "phone_jm") {
            countries.push(country::JM);
        }
        if cfg!(feature = "phone_jo") {
            countries.push(country::JO);
        }
        if cfg!(feature = "phone_jp") {
            countries.push(country::JP);
        }
        if cfg!(feature = "phone_ke") {
            countries.push(country::KE);
        }
        if cfg!(feature = "phone_kg") {
            countries.push(country::KG);
        }
        if cfg!(feature = "phone_kh") {
            countries.push(country::KH);
        }
        if cfg!(feature = "phone_ki") {
            countries.push(country::KI);
        }
        if cfg!(feature = "phone_km") {
            countries.push(country::KM);
        }
        if cfg!(feature = "phone_kn") {
            countries.push(country::KN);
        }
        if cfg!(feature = "phone_kp") {
            countries.push(country::KP);
        }
        if cfg!(feature = "phone_kr") {
            countries.push(country::KR);
        }
        if cfg!(feature = "phone_kw") {
            countries.push(country::KW);
        }
        if cfg!(feature = "phone_ky") {
            countries.push(country::KY);
        }
        if cfg!(feature = "phone_kz") {
            countries.push(country::KZ);
        }
        if cfg!(feature = "phone_la") {
            countries.push(country::LA);
        }
        if cfg!(feature = "phone_lb") {
            countries.push(country::LB);
        }
        if cfg!(feature = "phone_lc") {
            countries.push(country::LC);
        }
        if cfg!(feature = "phone_li") {
            countries.push(country::LI);
        }
        if cfg!(feature = "phone_lk") {
            countries.push(country::LK);
        }
        if cfg!(feature = "phone_lr") {
            countries.push(country::LR);
        }
        if cfg!(feature = "phone_ls") {
            countries.push(country::LS);
        }
        if cfg!(feature = "phone_lt") {
            countries.push(country::LT);
        }
        if cfg!(feature = "phone_lu") {
            countries.push(country::LU);
        }
        if cfg!(feature = "phone_lv") {
            countries.push(country::LV);
        }
        if cfg!(feature = "phone_ly") {
            countries.push(country::LY);
        }
        if cfg!(feature = "phone_ma") {
            countries.push(country::MA);
        }
        if cfg!(feature = "phone_mc") {
            countries.push(country::MC);
        }
        if cfg!(feature = "phone_md") {
            countries.push(country::MD);
        }
        if cfg!(feature = "phone_me") {
            countries.push(country::ME);
        }
        if cfg!(feature = "phone_mf") {
            countries.push(country::MF);
        }
        if cfg!(feature = "phone_mg") {
            countries.push(country::MG);
        }
        if cfg!(feature = "phone_mh") {
            countries.push(country::MH);
        }
        if cfg!(feature = "phone_mk") {
            countries.push(country::MK);
        }
        if cfg!(feature = "phone_ml") {
            countries.push(country::ML);
        }
        if cfg!(feature = "phone_mm") {
            countries.push(country::MM);
        }
        if cfg!(feature = "phone_mn") {
            countries.push(country::MN);
        }
        if cfg!(feature = "phone_mo") {
            countries.push(country::MO);
        }
        if cfg!(feature = "phone_mp") {
            countries.push(country::MP);
        }
        if cfg!(feature = "phone_mq") {
            countries.push(country::MQ);
        }
        if cfg!(feature = "phone_mr") {
            countries.push(country::MR);
        }
        if cfg!(feature = "phone_ms") {
            countries.push(country::MS);
        }
        if cfg!(feature = "phone_mt") {
            countries.push(country::MT);
        }
        if cfg!(feature = "phone_mu") {
            countries.push(country::MU);
        }
        if cfg!(feature = "phone_mv") {
            countries.push(country::MV);
        }
        if cfg!(feature = "phone_mw") {
            countries.push(country::MW);
        }
        if cfg!(feature = "phone_mx") {
            countries.push(country::MX);
        }
        if cfg!(feature = "phone_my") {
            countries.push(country::MY);
        }
        if cfg!(feature = "phone_mz") {
            countries.push(country::MZ);
        }
        if cfg!(feature = "phone_na") {
            countries.push(country::NA);
        }
        if cfg!(feature = "phone_nc") {
            countries.push(country::NC);
        }
        if cfg!(feature = "phone_ne") {
            countries.push(country::NE);
        }
        if cfg!(feature = "phone_nf") {
            countries.push(country::NF);
        }
        if cfg!(feature = "phone_ng") {
            countries.push(country::NG);
        }
        if cfg!(feature = "phone_ni") {
            countries.push(country::NI);
        }
        if cfg!(feature = "phone_nl") {
            countries.push(country::NL);
        }
        if cfg!(feature = "phone_no") {
            countries.push(country::NO);
        }
        if cfg!(feature = "phone_np") {
            countries.push(country::NP);
        }
        if cfg!(feature = "phone_nr") {
            countries.push(country::NR);
        }
        if cfg!(feature = "phone_nu") {
            countries.push(country::NU);
        }
        if cfg!(feature = "phone_nz") {
            countries.push(country::NZ);
        }
        if cfg!(feature = "phone_om") {
            countries.push(country::OM);
        }
        if cfg!(feature = "phone_pa") {
            countries.push(country::PA);
        }
        if cfg!(feature = "phone_pe") {
            countries.push(country::PE);
        }
        if cfg!(feature = "phone_pf") {
            countries.push(country::PF);
        }
        if cfg!(feature = "phone_pg") {
            countries.push(country::PG);
        }
        if cfg!(feature = "phone_ph") {
            countries.push(country::PH);
        }
        if cfg!(feature = "phone_pk") {
            countries.push(country::PK);
        }
        if cfg!(feature = "phone_pl") {
            countries.push(country::PL);
        }
        if cfg!(feature = "phone_pm") {
            countries.push(country::PM);
        }
        if cfg!(feature = "phone_pr") {
            countries.push(country::PR);
        }
        if cfg!(feature = "phone_ps") {
            countries.push(country::PS);
        }
        if cfg!(feature = "phone_pt") {
            countries.push(country::PT);
        }
        if cfg!(feature = "phone_pw") {
            countries.push(country::PW);
        }
        if cfg!(feature = "phone_py") {
            countries.push(country::PY);
        }
        if cfg!(feature = "phone_qa") {
            countries.push(country::QA);
        }
        if cfg!(feature = "phone_re") {
            countries.push(country::RE);
        }
        if cfg!(feature = "phone_ro") {
            countries.push(country::RO);
        }
        if cfg!(feature = "phone_rs") {
            countries.push(country::RS);
        }
        if cfg!(feature = "phone_ru") {
            countries.push(country::RU);
        }
        if cfg!(feature = "phone_rw") {
            countries.push(country::RW);
        }
        if cfg!(feature = "phone_sa") {
            countries.push(country::SA);
        }
        if cfg!(feature = "phone_sb") {
            countries.push(country::SB);
        }
        if cfg!(feature = "phone_sc") {
            countries.push(country::SC);
        }
        if cfg!(feature = "phone_sd") {
            countries.push(country::SD);
        }
        if cfg!(feature = "phone_se") {
            countries.push(country::SE);
        }
        if cfg!(feature = "phone_sg") {
            countries.push(country::SG);
        }
        if cfg!(feature = "phone_sh") {
            countries.push(country::SH);
        }
        if cfg!(feature = "phone_si") {
            countries.push(country::SI);
        }
        if cfg!(feature = "phone_sj") {
            countries.push(country::SJ);
        }
        if cfg!(feature = "phone_sk") {
            countries.push(country::SK);
        }
        if cfg!(feature = "phone_sl") {
            countries.push(country::SL);
        }
        if cfg!(feature = "phone_sm") {
            countries.push(country::SM);
        }
        if cfg!(feature = "phone_sn") {
            countries.push(country::SN);
        }
        if cfg!(feature = "phone_so") {
            countries.push(country::SO);
        }
        if cfg!(feature = "phone_sr") {
            countries.push(country::SR);
        }
        if cfg!(feature = "phone_ss") {
            countries.push(country::SS);
        }
        if cfg!(feature = "phone_st") {
            countries.push(country::ST);
        }
        if cfg!(feature = "phone_sv") {
            countries.push(country::SV);
        }
        if cfg!(feature = "phone_sx") {
            countries.push(country::SX);
        }
        if cfg!(feature = "phone_sy") {
            countries.push(country::SY);
        }
        if cfg!(feature = "phone_sz") {
            countries.push(country::SZ);
        }
        if cfg!(feature = "phone_ta") {
            countries.push(country::TA);
        }
        if cfg!(feature = "phone_tc") {
            countries.push(country::TC);
        }
        if cfg!(feature = "phone_td") {
            countries.push(country::TD);
        }
        if cfg!(feature = "phone_tg") {
            countries.push(country::TG);
        }
        if cfg!(feature = "phone_th") {
            countries.push(country::TH);
        }
        if cfg!(feature = "phone_tj") {
            countries.push(country::TJ);
        }
        if cfg!(feature = "phone_tk") {
            countries.push(country::TK);
        }
        if cfg!(feature = "phone_tl") {
            countries.push(country::TL);
        }
        if cfg!(feature = "phone_tm") {
            countries.push(country::TM);
        }
        if cfg!(feature = "phone_tn") {
            countries.push(country::TN);
        }
        if cfg!(feature = "phone_to") {
            countries.push(country::TO);
        }
        if cfg!(feature = "phone_tr") {
            countries.push(country::TR);
        }
        if cfg!(feature = "phone_tt") {
            countries.push(country::TT);
        }
        if cfg!(feature = "phone_tv") {
            countries.push(country::TV);
        }
        if cfg!(feature = "phone_tw") {
            countries.push(country::TW);
        }
        if cfg!(feature = "phone_tz") {
            countries.push(country::TZ);
        }
        if cfg!(feature = "phone_ua") {
            countries.push(country::UA);
        }
        if cfg!(feature = "phone_ug") {
            countries.push(country::UG);
        }
        if cfg!(feature = "phone_us") {
            countries.push(country::US);
        }
        if cfg!(feature = "phone_uy") {
            countries.push(country::UY);
        }
        if cfg!(feature = "phone_uz") {
            countries.push(country::UZ);
        }
        if cfg!(feature = "phone_va") {
            countries.push(country::VA);
        }
        if cfg!(feature = "phone_vc") {
            countries.push(country::VC);
        }
        if cfg!(feature = "phone_ve") {
            countries.push(country::VE);
        }
        if cfg!(feature = "phone_vg") {
            countries.push(country::VG);
        }
        if cfg!(feature = "phone_vi") {
            countries.push(country::VI);
        }
        if cfg!(feature = "phone_vn") {
            countries.push(country::VN);
        }
        if cfg!(feature = "phone_vu") {
            countries.push(country::VU);
        }
        if cfg!(feature = "phone_wf") {
            countries.push(country::WF);
        }
        if cfg!(feature = "phone_ws") {
            countries.push(country::WS);
        }
        if cfg!(feature = "phone_ye") {
            countries.push(country::YE);
        }
        if cfg!(feature = "phone_yt") {
            countries.push(country::YT);
        }
        if cfg!(feature = "phone_za") {
            countries.push(country::ZA);
        }
        if cfg!(feature = "phone_zm") {
            countries.push(country::ZM);
        }
        if cfg!(feature = "phone_zw") {
            countries.push(country::ZW);
        }

        countries
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum PhoneNumberError {
    IncorrectFormat,
}

impl Display for PhoneNumberError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for PhoneNumberError {}

pub type PhoneNumberResult = Result<PhoneNumber, PhoneNumberError>;

#[derive(Debug, PartialEq)]
pub struct PhoneNumberValidator {}

#[derive(Clone, PartialEq, Eq)]
pub struct PhoneNumber {
    full_phone_number: String,
    countries: Vec<country::Id>,
}

impl PhoneNumber {
    pub fn get_countries(&self) -> &[country::Id] {
        &self.countries
    }

    pub fn get_full_phone_number(&self) -> &str {
        &self.full_phone_number
    }

    pub fn into_string(self) -> String {
        self.full_phone_number
    }
}

impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.full_phone_number
    }
}

impl Validated for PhoneNumber {}

impl Debug for PhoneNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("PhoneNumber({})", self.full_phone_number))?;
        Ok(())
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_phone_number)?;
        Ok(())
    }
}

impl Hash for PhoneNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_phone_number.hash(state);
    }
}

impl PhoneNumberValidator {
    pub fn is_phone_number(&self, full_phone_number: &str) -> bool {
        self.parse_inner(full_phone_number).is_ok()
    }

    pub fn parse_string(&self, full_phone_number: String) -> PhoneNumberResult {
        let mut phone_number_inner = self.parse_inner(&full_phone_number)?;

        phone_number_inner.full_phone_number = full_phone_number;

        Ok(phone_number_inner)
    }

    pub fn parse_str(&self, full_phone_number: &str) -> PhoneNumberResult {
        let mut phone_number_inner = self.parse_inner(full_phone_number)?;

        phone_number_inner.full_phone_number.push_str(full_phone_number);

        Ok(phone_number_inner)
    }

    fn parse_inner(&self, full_phone_number: &str) -> PhoneNumberResult {
        let mut countries = Vec::new();

        for &country in COUNTRIES.iter() {
            if let Ok(phone_number) = phonenumber::parse(Some(country), full_phone_number) {
                if phone_number.is_valid() {
                    countries.push(country);
                }
            }
        }

        if countries.is_empty() {
            Err(PhoneNumberError::IncorrectFormat)
        } else {
            Ok(PhoneNumber {
                full_phone_number: String::new(),
                countries: countries,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_number_methods() {
        let phone_number = "0912345678".to_string();

        let pnv = PhoneNumberValidator {};

        let phone_number = pnv.parse_string(phone_number).unwrap();

        assert_eq!("0912345678", phone_number.get_full_phone_number());
        assert!(phone_number.get_countries().contains(&country::TW));
    }

    #[test]
    fn test_phone_number_lv1() {
        let phone_number = "0912345678".to_string();

        let pnv = PhoneNumberValidator {};

        pnv.parse_string(phone_number).unwrap();
    }
}

// PhoneNumber's wrapper struct is itself
impl ValidatedWrapper for PhoneNumber {
    type Error = PhoneNumberError;

    fn from_string(phone_number: String) -> Result<Self, Self::Error> {
        PhoneNumber::from_string(phone_number)
    }

    fn from_str(phone_number: &str) -> Result<Self, Self::Error> {
        PhoneNumber::from_str(phone_number)
    }
}

impl PhoneNumber {
    pub fn from_string(phone_number: String) -> Result<Self, PhoneNumberError> {
        PhoneNumber::create_validator().parse_string(phone_number)
    }

    pub fn from_str(phone_number: &str) -> Result<Self, PhoneNumberError> {
        PhoneNumber::create_validator().parse_str(phone_number)
    }

    fn create_validator() -> PhoneNumberValidator {
        PhoneNumberValidator {}
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromParam<'a> for PhoneNumber {
    type Error = PhoneNumberError;

    fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        PhoneNumber::from_str(param)
    }
}

#[cfg(feature = "rocketly")]
impl<'a> ::rocket::request::FromFormValue<'a> for PhoneNumber {
    type Error = PhoneNumberError;

    fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
        PhoneNumber::from_str(form_value)
    }
}

#[cfg(feature = "serdely")]
struct StringVisitor;

#[cfg(feature = "serdely")]
impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
    type Value = PhoneNumber;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a PhoneNumber string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
        PhoneNumber::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
        PhoneNumber::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
impl<'de> ::serde::Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

#[cfg(feature = "serdely")]
impl ::serde::Serialize for PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        serializer.serialize_str(&self.full_phone_number)
    }
}