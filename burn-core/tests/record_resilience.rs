#[cfg(feature = "std")]
mod tests {
    use burn::{
        module::Module,
        nn,
        record::{
            BinFileRecorder, DefaultFileRecorder, FileRecorder, FullPrecisionSettings,
            PrettyJsonFileRecorder, RecorderError,
        },
    };
    use burn_core as burn;
    use burn_tensor::backend::Backend;
    use std::path::PathBuf;

    type TestBackend = burn_ndarray::NdArray<f32>;

    #[derive(Module, Debug)]
    pub struct Model<B: Backend> {
        single_const: f32,
        linear1: nn::Linear<B>,
        array_const: [usize; 2],
        linear2: nn::Linear<B>,
    }

    #[derive(Module, Debug)]
    pub struct ModelNewOptionalField<B: Backend> {
        single_const: f32,
        linear1: nn::Linear<B>,
        array_const: [usize; 2],
        linear2: nn::Linear<B>,
        new_field: Option<usize>,
    }

    #[derive(Module, Debug)]
    pub struct ModelNewConstantField<B: Backend> {
        single_const: f32,
        linear1: nn::Linear<B>,
        array_const: [usize; 2],
        linear2: nn::Linear<B>,
        new_field: usize,
    }

    #[derive(Module, Debug)]
    pub struct ModelNewFieldOrders<B: Backend> {
        array_const: [usize; 2],
        linear2: nn::Linear<B>,
        single_const: f32,
        linear1: nn::Linear<B>,
    }

    #[test]
    fn deserialize_with_new_optional_field_works_with_default_file_recorder() {
        deserialize_with_new_optional_field(
            "default",
            DefaultFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_removed_optional_field_works_with_default_file_recorder() {
        deserialize_with_removed_optional_field(
            "default",
            DefaultFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_new_constant_field_works_with_default_file_recorder() {
        deserialize_with_new_constant_field(
            "default",
            DefaultFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_removed_constant_field_works_with_default_file_recorder() {
        deserialize_with_removed_constant_field(
            "default",
            DefaultFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_new_field_order_works_with_default_file_recorder() {
        deserialize_with_new_field_order(
            "default",
            DefaultFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }
    #[test]
    fn deserialize_with_new_optional_field_works_with_pretty_json() {
        deserialize_with_new_optional_field(
            "pretty-json",
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_removed_optional_field_works_with_pretty_json() {
        deserialize_with_removed_optional_field(
            "pretty-json",
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_new_constant_field_works_with_pretty_json() {
        deserialize_with_new_constant_field(
            "pretty-json",
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_removed_constant_field_works_with_pretty_json() {
        deserialize_with_removed_constant_field(
            "pretty-json",
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_new_field_order_works_with_pretty_json() {
        deserialize_with_new_field_order(
            "pretty-json",
            PrettyJsonFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_with_new_optional_field_doesnt_works_with_bin_file_recorder() {
        deserialize_with_new_optional_field("bin", BinFileRecorder::<FullPrecisionSettings>::new())
            .unwrap();
    }

    #[test]
    fn deserialize_with_removed_optional_field_works_with_bin_file_recorder() {
        deserialize_with_removed_optional_field(
            "bin",
            BinFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    fn deserialize_with_new_constant_field_works_with_bin_file_recorder() {
        deserialize_with_new_constant_field("bin", BinFileRecorder::<FullPrecisionSettings>::new())
            .unwrap();
    }

    #[test]
    fn deserialize_with_removed_constant_field_works_with_bin_file_recorder() {
        deserialize_with_removed_constant_field(
            "bin",
            BinFileRecorder::<FullPrecisionSettings>::new(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn deserialize_with_new_field_order_works_with_bin_file_recorder() {
        deserialize_with_new_field_order("bin", BinFileRecorder::<FullPrecisionSettings>::new())
            .unwrap();
    }

    #[inline(always)]
    fn file_path(filename: String) -> PathBuf {
        std::env::temp_dir().join(filename)
    }

    fn deserialize_with_new_optional_field<R>(name: &str, recorder: R) -> Result<(), RecorderError>
    where
        R: FileRecorder,
    {
        let file_path: PathBuf = file_path(format!("deserialize_with_new_optional_field-{name}"));
        let model = Model {
            single_const: 32.0,
            linear1: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            array_const: [2, 2],
            linear2: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
        };

        recorder
            .record(model.into_record(), file_path.clone())
            .unwrap();
        let result = recorder.load::<ModelNewOptionalFieldRecord<TestBackend>>(file_path.clone());
        std::fs::remove_file(file_path).ok();

        result?;
        Ok(())
    }

    fn deserialize_with_removed_optional_field<R>(
        name: &str,
        recorder: R,
    ) -> Result<(), RecorderError>
    where
        R: FileRecorder,
    {
        let file_path: PathBuf =
            file_path(format!("deserialize_with_removed_optional_field-{name}"));
        let model = ModelNewOptionalField {
            single_const: 32.0,
            linear1: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            array_const: [2, 2],
            linear2: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            new_field: None,
        };

        recorder
            .record(model.into_record(), file_path.clone())
            .unwrap();
        let result = recorder.load::<ModelRecord<TestBackend>>(file_path.clone());
        std::fs::remove_file(file_path).ok();

        result?;
        Ok(())
    }

    fn deserialize_with_new_constant_field<R>(name: &str, recorder: R) -> Result<(), RecorderError>
    where
        R: FileRecorder,
    {
        let file_path: PathBuf = file_path(format!("deserialize_with_new_constant_field-{name}"));
        let model = Model {
            single_const: 32.0,
            array_const: [2, 2],
            linear1: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            linear2: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
        };

        recorder
            .record(model.into_record(), file_path.clone())
            .unwrap();
        let result = recorder.load::<ModelNewConstantFieldRecord<TestBackend>>(file_path.clone());
        std::fs::remove_file(file_path).ok();

        result?;
        Ok(())
    }

    fn deserialize_with_removed_constant_field<R>(
        name: &str,
        recorder: R,
    ) -> Result<(), RecorderError>
    where
        R: FileRecorder,
    {
        let file_path: PathBuf =
            file_path(format!("deserialize_with_removed_constant_field-{name}"));
        let model = ModelNewConstantField {
            single_const: 32.0,
            array_const: [2, 2],
            linear1: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            linear2: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            new_field: 0,
        };

        recorder
            .record(model.into_record(), file_path.clone())
            .unwrap();
        let result = recorder.load::<ModelRecord<TestBackend>>(file_path.clone());
        std::fs::remove_file(file_path).ok();

        result?;
        Ok(())
    }

    fn deserialize_with_new_field_order<R>(name: &str, recorder: R) -> Result<(), RecorderError>
    where
        R: FileRecorder,
    {
        let file_path: PathBuf = file_path(format!("deserialize_with_new_field_order-{name}"));
        let model = Model {
            array_const: [2, 2],
            single_const: 32.0,
            linear1: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
            linear2: nn::LinearConfig::new(20, 20).init::<TestBackend>(),
        };

        recorder
            .record(model.into_record(), file_path.clone())
            .unwrap();

        let result = recorder.load::<ModelNewFieldOrdersRecord<TestBackend>>(file_path.clone());
        std::fs::remove_file(file_path).ok();

        result?;
        Ok(())
    }
}
