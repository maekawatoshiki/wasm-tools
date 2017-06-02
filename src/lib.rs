/* Copyright 2017 Mozilla Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub use parser::Parser;
pub use parser::ParserState;
pub use parser::SectionCode;
pub use parser::Operator;
pub use parser::Type;
pub use parser::CustomSectionKind;
pub use parser::NameType;
pub use parser::Naming;
pub use parser::LocalName;
pub use parser::NameEntry;
pub use parser::ExternalKind;
pub use parser::FuncType;
pub use parser::ResizableLimits;
pub use parser::TableType;
pub use parser::MemoryType;
pub use parser::GlobalType;
pub use parser::MemoryImmediate;
pub use parser::BrTable;
pub use parser::ImportSectionEntryType;

mod parser;
mod tests;