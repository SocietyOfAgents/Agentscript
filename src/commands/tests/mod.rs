use crate::model::Rule;
use crate::policy::parse;

#[test]
fn parses_preserve_function_policy() {
    let policy = parse(
        "policy payments {\n\
         checkpoint baseline\n\
         preserve --function GatewayService.call\n\
         }\n",
    )
    .expect("valid policy");

    assert_eq!(policy.name, "payments");
    assert_eq!(policy.checkpoint, "baseline");
    assert!(matches!(
        &policy.rules[0],
        Rule::PreserveFunction { target } if target == "GatewayService.call"
    ));
}

#[test]
fn rejects_missing_checkpoint() {
    let error = parse("policy payments {\npreserve --function call\n}\n")
        .expect_err("checkpoint is required");
    assert_eq!(error, "MVP requires an explicit checkpoint");
}
