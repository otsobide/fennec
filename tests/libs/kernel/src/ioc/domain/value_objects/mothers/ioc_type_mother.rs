use kernel::ioc::domain::value_objects::ioc_type::IocType;

pub struct IocTypeMother;

#[allow(dead_code)]
impl IocTypeMother {
    pub fn ipv4() -> IocType {
        IocType::Ipv4
    }

    pub fn ipv6() -> IocType {
        IocType::Ipv6
    }

    pub fn domain() -> IocType {
        IocType::Domain
    }

    pub fn url() -> IocType {
        IocType::Url
    }

    pub fn sha256() -> IocType {
        IocType::Sha256
    }

    pub fn sha1() -> IocType {
        IocType::Sha1
    }

    pub fn md5() -> IocType {
        IocType::Md5
    }

    pub fn email() -> IocType {
        IocType::Email
    }

    pub fn random() -> IocType {
        IocType::Ipv4
    }
}
