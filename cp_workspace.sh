cp -R ./openapi/src ./bot-core/
mv ./bot-core/src ./bot-core/openapi
mv ./bot-core/openapi/lib.rs ./bot-core/openapi/mod.rs

find ./bot-core/openapi -name "*.rs" | xargs sed -i -e 's/openapi::/crate::/g' -e 's/crate::/crate::openapi::/g'
