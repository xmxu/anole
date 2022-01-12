
#[cfg(test)]
mod xml {

    #[test]
    fn test_xml_de() {
        env_logger::init();
        use anole::de::xml;
        let xml = r#"
        <rsp>
            <order>DKKII939030303</order>
            <author count="3">
                <name>kk</name>
                <name>uu</name>
                <name>vv</name>
            </author>
        </rsp>
        "#;
        let de = xml::De::get(xml, "order").unwrap();
        assert_eq!(de.as_str(), "DKKII939030303");
        let de = xml::De::get(xml, "author.name").unwrap();
        assert_eq!(de.as_str(), "kk");
        let de = xml::De::get(xml, "order#count").unwrap();
        assert_eq!(de.as_str(), "3");
    }
}