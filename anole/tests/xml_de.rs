
#[cfg(test)]
mod xml {

    #[test]
    fn test_xml_de() {
        env_logger::init();
        use anole::de::xml;
        let xml = r#"
        <rsp>
            <order>DKKII939030303</order>
            <author count="3" date="20210113">
                <name id="1">kk</name>
                <name>uu</name>
                <name>vv</name>
            </author>
        </rsp>
        "#;
        let de = xml::De::get(xml, "order").unwrap();
        assert_eq!(de.as_str(), "DKKII939030303");
        let de = xml::De::get(xml, "author.name|2").unwrap();
        assert_eq!(de.as_str(), "vv");
        assert!(xml::De::get(xml, "author.name|3").is_err());
        //unimplement
        // assert_eq!(xml::De::get(xml, "author.name|0#id").unwrap().as_str(), "1");
        let de = xml::De::get(xml, "rsp.author#count").unwrap();
        assert_eq!(de.as_str(), "3");
    }
}