// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_usage_breakdown<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::UsageBreakdown>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::UsageBreakdownBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "resourceType" => {
                                builder = builder.set_resource_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| crate::types::ResourceType::from(u.as_ref())))
                                        .transpose()?,
                                );
                            },
                            "currentUsage" => {
                                builder = builder.set_current_usage(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i32::try_from)
                                        .transpose()?,
                                );
                            },
                            "currentOverages" => {
                                builder = builder.set_current_overages(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i32::try_from)
                                        .transpose()?,
                                );
                            },
                            "usageLimit" => {
                                builder = builder.set_usage_limit(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i32::try_from)
                                        .transpose()?,
                                );
                            },
                            "unit" => {
                                builder = builder.set_unit(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            },
                            "overageCharges" => {
                                builder = builder.set_overage_charges(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(|v| v.to_f64_lossy()),
                                );
                            },
                            "currency" => {
                                builder = builder.set_currency(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| crate::types::Currency::from(u.as_ref())))
                                        .transpose()?,
                                );
                            },
                            "overageRate" => {
                                builder = builder.set_overage_rate(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(|v| v.to_f64_lossy()),
                                );
                            },
                            "nextDateReset" => {
                                builder = builder.set_next_date_reset(
                                    ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                        tokens.next(),
                                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                                    )?,
                                );
                            },
                            "overageCap" => {
                                builder = builder.set_overage_cap(
                                    ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                        .map(i32::try_from)
                                        .transpose()?,
                                );
                            },
                            "freeTrialInfo" => {
                                builder = builder.set_free_trial_info(
                                    crate::protocol_serde::shape_free_trial_info::de_free_trial_info(tokens)?,
                                );
                            },
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            format!("expected object key or end object, found: {:?}", other),
                        ));
                    },
                }
            }
            Ok(Some(
                crate::serde_util::usage_breakdown_correct_errors(builder)
                    .build()
                    .map_err(|err| {
                        ::aws_smithy_json::deserialize::error::DeserializeError::custom_source(
                            "Response was invalid",
                            err,
                        )
                    })?,
            ))
        },
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
