use derive_builder::Builder;
#[derive(Debug, Clone, Builder, Default)]
pub struct ViperServerOpts {
    #[doc = "`--backendSpecificCache`\n\nUse a separate cache for each backend?"]
    #[builder(setter(strip_option), default)]
    pub backend_specific_cache: Option<bool>,
    #[doc = "`--cacheFile  <arg>`\n\nSpecifies the file from which the cache gets initialized on startup and to which the resulting cache gets written to.If it is not set, the cache will be initially empty and is only kept in memory, so it is not persisted during runs"]
    #[builder(setter(into, strip_option), default)]
    pub cache_file: Option<String>,
    #[doc = "`--disableVersionCheck`\n\nDisables the client's version check."]
    #[builder(setter(strip_option), default)]
    pub disable_version_check: Option<bool>,
    #[doc = "`--logFile  <arg>`\n\nSpecifies the location of the log file to be used by ViperServer and the verification backends it creates."]
    #[builder(setter(into, strip_option), default)]
    pub log_file: Option<String>,
    #[doc = "`--logLevel  <arg>`\n\nOne of the log levels: ALL,TRACE,DEBUG,INFO,WARN,ERROR,OFF."]
    #[builder(setter(into, strip_option), default)]
    pub log_level: Option<String>,
    #[doc = "`-m, --maximumActiveJobs  <arg>`\n\nSpecifies the maximal amount of jobs that may run concurrently.The number must be positive integer.If the option is omitted, a default number of 3 jobs will be set."]
    #[builder(setter(into, strip_option), default)]
    pub maximum_active_jobs: Option<String>,
    #[doc = "`--nThreads  <arg>`\n\nMaximal number of threads that should be used (not taking threads used by backend into account) Values below 3 (the minimum) will be set to the minimum. The default value is the maximum of 3 and the number of available processors"]
    #[builder(setter(into, strip_option), default)]
    pub n_threads: Option<String>,
    #[doc = "`-p, --port  <arg>`\n\nSpecifies the port on which ViperServer will be started.The port must be an integer in range [1100-65535]If the option is omitted, an available port will be selected automatically."]
    #[builder(setter(into, strip_option), default)]
    pub port: Option<String>,
    #[doc = "`--serverMode  <arg>`\n\nOne of the supported protocols: LSP,HTTP."]
    #[builder(setter(into, strip_option), default)]
    pub server_mode: Option<String>,
    #[doc = "`--singleClient`\n\nHandles only a single client in LSP mode and terminates automatically afterwards"]
    #[builder(setter(strip_option), default)]
    pub single_client: Option<bool>,
}
impl std::fmt::Display for ViperServerOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(true) = &self.backend_specific_cache {
            write!(f, "--backendSpecificCache ")?;
        }
        if let Some(value) = &self.cache_file {
            write!(f, "--cacheFile {value} ")?;
        }
        if let Some(true) = &self.disable_version_check {
            write!(f, "--disableVersionCheck ")?;
        }
        if let Some(value) = &self.log_file {
            write!(f, "--logFile {value} ")?;
        }
        if let Some(value) = &self.log_level {
            write!(f, "--logLevel {value} ")?;
        }
        if let Some(value) = &self.maximum_active_jobs {
            write!(f, "--maximumActiveJobs {value} ")?;
        }
        if let Some(value) = &self.n_threads {
            write!(f, "--nThreads {value} ")?;
        }
        if let Some(value) = &self.port {
            write!(f, "--port {value} ")?;
        }
        if let Some(value) = &self.server_mode {
            write!(f, "--serverMode {value} ")?;
        }
        if let Some(true) = &self.single_client {
            write!(f, "--singleClient ")?;
        }
        Ok(())
    }
}
impl ViperServerOpts {
    pub fn apply(&self, mut f: impl FnMut(&str)) {
        if let Some(true) = &self.backend_specific_cache {
            f("--backendSpecificCache");
        }
        if let Some(value) = &self.cache_file {
            f("--cacheFile");
            f(value);
        }
        if let Some(true) = &self.disable_version_check {
            f("--disableVersionCheck");
        }
        if let Some(value) = &self.log_file {
            f("--logFile");
            f(value);
        }
        if let Some(value) = &self.log_level {
            f("--logLevel");
            f(value);
        }
        if let Some(value) = &self.maximum_active_jobs {
            f("--maximumActiveJobs");
            f(value);
        }
        if let Some(value) = &self.n_threads {
            f("--nThreads");
            f(value);
        }
        if let Some(value) = &self.port {
            f("--port");
            f(value);
        }
        if let Some(value) = &self.server_mode {
            f("--serverMode");
            f(value);
        }
        if let Some(true) = &self.single_client {
            f("--singleClient");
        }
    }
}
#[derive(Debug, Clone, Builder, Default)]
pub struct CarbonOpts {
    #[doc = "`--assumeInjectivityOnInhale`\n\nAssumes injectivity of the receiver expression when inhaling quantified permissions, instead of checking it."]
    #[builder(setter(strip_option), default)]
    pub assume_injectivity_on_inhale: Option<bool>,
    #[doc = "`--boogieExe  <arg>`\n\nManually-specified full path to Boogie.exe executable (default: ${BOOGIE_EXE})"]
    #[builder(setter(into, strip_option), default)]
    pub boogie_exe: Option<String>,
    #[doc = "`--boogieOpt  <arg>`\n\nOption(s) to pass-through as options to Boogie (changing the output generated by Boogie is not supported) (default: none)"]
    #[builder(setter(into, strip_option), default)]
    pub boogie_opt: Option<String>,
    #[doc = "`--counterexample  <arg>`\n\nReturn counterexample for errors. Pass 'native' for returning the native model from the backend, 'variables' for returning a model of all local Viper variables, or 'mapped' (only available on Silicon) for returning a model with Ref variables resolved to object-like structures."]
    #[builder(setter(into, strip_option), default)]
    pub counterexample: Option<String>,
    #[doc = "`--disableAllocEncoding`\n\nDisable Allocation-related assumptions (default: enabled)"]
    #[builder(setter(strip_option), default)]
    pub disable_alloc_encoding: Option<bool>,
    #[doc = "`--plugin  <arg>`\n\nLoad plugin(s) with given class name(s). Several plugins can be separated by ':'. The fully qualified class name of the plugin should be specified."]
    #[builder(setter(into, strip_option), default)]
    pub plugin: Option<String>,
    #[doc = "`--print  <arg>`\n\nWrite the Boogie output file to the provided filename (default: none)"]
    #[builder(setter(into, strip_option), default)]
    pub print: Option<String>,
    #[doc = "`--proverLog  <arg>`\n\nProver log file written by Boogie (default: none)"]
    #[builder(setter(into, strip_option), default)]
    pub prover_log: Option<String>,
    #[doc = "`--z3Exe  <arg>`\n\nManually-specified full path to Z3.exe executable (default: ${Z3_EXE})"]
    #[builder(setter(into, strip_option), default)]
    pub z3_exe: Option<String>,
}
impl std::fmt::Display for CarbonOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(true) = &self.assume_injectivity_on_inhale {
            write!(f, "--assumeInjectivityOnInhale ")?;
        }
        if let Some(value) = &self.boogie_exe {
            write!(f, "--boogieExe {value} ")?;
        }
        if let Some(value) = &self.boogie_opt {
            write!(f, "--boogieOpt {value} ")?;
        }
        if let Some(value) = &self.counterexample {
            write!(f, "--counterexample {value} ")?;
        }
        if let Some(true) = &self.disable_alloc_encoding {
            write!(f, "--disableAllocEncoding ")?;
        }
        if let Some(value) = &self.plugin {
            write!(f, "--plugin {value} ")?;
        }
        if let Some(value) = &self.print {
            write!(f, "--print {value} ")?;
        }
        if let Some(value) = &self.prover_log {
            write!(f, "--proverLog {value} ")?;
        }
        if let Some(value) = &self.z3_exe {
            write!(f, "--z3Exe {value} ")?;
        }
        Ok(())
    }
}
impl CarbonOpts {
    pub fn apply(&self, mut f: impl FnMut(&str)) {
        if let Some(true) = &self.assume_injectivity_on_inhale {
            f("--assumeInjectivityOnInhale");
        }
        if let Some(value) = &self.boogie_exe {
            f("--boogieExe");
            f(value);
        }
        if let Some(value) = &self.boogie_opt {
            f("--boogieOpt");
            f(value);
        }
        if let Some(value) = &self.counterexample {
            f("--counterexample");
            f(value);
        }
        if let Some(true) = &self.disable_alloc_encoding {
            f("--disableAllocEncoding");
        }
        if let Some(value) = &self.plugin {
            f("--plugin");
            f(value);
        }
        if let Some(value) = &self.print {
            f("--print");
            f(value);
        }
        if let Some(value) = &self.prover_log {
            f("--proverLog");
            f(value);
        }
        if let Some(value) = &self.z3_exe {
            f("--z3Exe");
            f(value);
        }
    }
}
#[derive(Debug, Clone, Builder, Default)]
pub struct SiliconOpts {
    #[doc = "`--alternativeFunctionVerificationOrder`\n\nCalculate the order in which functions are verified and function axioms become available in an alternative way that takes dependencies between functions through predicate unfoldings into account. This is more complete in some cases (see Silicon issue #355) but less complete in others (see test all/issues/silicon/unofficial007)."]
    #[builder(setter(strip_option), default)]
    pub alternative_function_verification_order: Option<bool>,
    #[doc = "`--assertionMode  <arg>...`\n\nDetermines how assertion checks are encoded in SMTLIB. Options are 'pp' (push-pop) and 'sc' (soft constraints) (default: pp)."]
    #[builder(setter(into, strip_option), default)]
    pub assertion_mode: Option<String>,
    #[doc = "`--assertTimeout  <arg>`\n\nTimeout (in ms) per SMT solver assertion (default: 0, i.e. no timeout). Ignored when using the cvc5 prover."]
    #[builder(setter(into, strip_option), default)]
    pub assert_timeout: Option<String>,
    #[doc = "`--assumeInjectivityOnInhale`\n\nAssumes injectivity of the receiver expression when inhaling quantified permissions, instead of checking it."]
    #[builder(setter(strip_option), default)]
    pub assume_injectivity_on_inhale: Option<bool>,
    #[doc = "`--checkTimeout  <arg>`\n\nTimeout (in ms) per SMT solver check. Solver checks differ from solver asserts in that a failing assert always yields a verification error whereas a failing check doesn't, at least not directly. However, failing checks might result in performance degradation, e.g. when a dead program path is nevertheless explored, and indirectly in verification failures due to incompletenesses, e.g. when the held permission amount is too coarsely underapproximated (default: 10). Ignored when using the cvc5 prover."]
    #[builder(setter(into, strip_option), default)]
    pub check_timeout: Option<String>,
    #[doc = "`--conditionalizePermissions`\n\nPotentially reduces the number of symbolic execution paths, by conditionalising permission expressions. E.g. rewrite \"b ==> acc(x.f, p)\" to \"acc(x.f, b ? p : none)\".This is an experimental feature; report problems if you observe any."]
    #[builder(setter(strip_option), default)]
    pub conditionalize_permissions: Option<bool>,
    #[doc = "`--counterexample  <arg>`\n\nReturn counterexample for errors. Pass 'native' for returning the native model from the backend, 'variables' for returning a model of all local Viper variables, or 'mapped' (only available on Silicon) for returning a model with Ref variables resolved to object-like structures."]
    #[builder(setter(into, strip_option), default)]
    pub counterexample: Option<String>,
    #[doc = "`--cvc5Exe  <arg>`\n\ncvc5 executable. The environment variable CVC5_EXE can also be used to specify the path of the executable."]
    #[builder(setter(into, strip_option), default)]
    pub cvc5_exe: Option<String>,
    #[doc = "`--disableCaches`\n\nDisables various caches in Silicon's state."]
    #[builder(setter(strip_option), default)]
    pub disable_caches: Option<bool>,
    #[doc = "`--disableCatchingExceptions`\n\nDon't catch exceptions (can be useful for debugging problems with Silicon)"]
    #[builder(setter(strip_option), default)]
    pub disable_catching_exceptions: Option<bool>,
    #[doc = "`--disableChunkOrderHeuristics`\n\nDisable heuristic ordering of quantified chunks (context: iterated separating conjunctions)."]
    #[builder(setter(strip_option), default)]
    pub disable_chunk_order_heuristics: Option<bool>,
    #[doc = "`--disableFunctionUnfoldTrigger`\n\nDisables automatic triggering of function definitions when unfolding predicates they depend on."]
    #[builder(setter(strip_option), default)]
    pub disable_function_unfold_trigger: Option<bool>,
    #[doc = "`--disableHavocHack407`\n\nA Viper method call to ___silicon_hack407_havoc_all_R, where R is a field or predicate, results in Silicon havocking all instances of R. See also Silicon issue #407."]
    #[builder(setter(strip_option), default)]
    pub disable_havoc_hack407: Option<bool>,
    #[doc = "`--disableISCTriggers`\n\nDon't pick triggers for quantifiers, let the SMT solver do it (context: iterated separating conjunctions)."]
    #[builder(setter(strip_option), default)]
    pub disable_isc_triggers: Option<bool>,
    #[doc = "`--disableShortCircuitingEvaluations`\n\nDisable short-circuiting evaluation of AND, OR. If disabled, evaluating e.g., i > 0 && f(i), will fail if f's precondition requires i > 0."]
    #[builder(setter(strip_option), default)]
    pub disable_short_circuiting_evaluations: Option<bool>,
    #[doc = "`--disableSubsumption`\n\nDon't add assumptions gained by verifying an assert statement"]
    #[builder(setter(strip_option), default)]
    pub disable_subsumption: Option<bool>,
    #[doc = "`--disableTempDirectory`\n\nDisable the creation of temporary data (default: ./tmp)"]
    #[builder(setter(strip_option), default)]
    pub disable_temp_directory: Option<bool>,
    #[doc = "`--disableValueMapCaching`\n\nDisable caching of value maps (context: iterated separating conjunctions)."]
    #[builder(setter(strip_option), default)]
    pub disable_value_map_caching: Option<bool>,
    #[doc = "`--enableBranchconditionReporting`\n\nReport branch conditions (can be useful for assertions that fail on multiple branches)"]
    #[builder(setter(strip_option), default)]
    pub enable_branchcondition_reporting: Option<bool>,
    #[doc = "`--enableMoreCompleteExhale`\n\nEnable a more complete exhale version."]
    #[builder(setter(strip_option), default)]
    pub enable_more_complete_exhale: Option<bool>,
    #[doc = "`--enablePredicateTriggersOnInhale`\n\nEmit predicate-based function trigger on each inhale of a predicate instance (context: heap-dependent functions)."]
    #[builder(setter(strip_option), default)]
    pub enable_predicate_triggers_on_inhale: Option<bool>,
    #[doc = "`--excludeMethods  <arg>`\n\nExclude methods from verification (default: ''). Is applied after the include pattern."]
    #[builder(setter(into, strip_option), default)]
    pub exclude_methods: Option<String>,
    #[doc = "`--handlePureConjunctsIndividually`\n\nHandle pure conjunction individually.Increases precision of error reporting, but may slow down verification."]
    #[builder(setter(strip_option), default)]
    pub handle_pure_conjuncts_individually: Option<bool>,
    #[doc = "`--includeMethods  <arg>`\n\nInclude methods in verification (default: '*'). Wildcard characters are '?' and '*'. -Llogger=level [logger=level]...                 Set level of certain internal loggers"]
    #[builder(setter(into, strip_option), default)]
    pub include_methods: Option<String>,
    #[doc = "`--logConfig  <arg>`\n\nPath to config file specifying SymbExLogger options"]
    #[builder(setter(into, strip_option), default)]
    pub log_config: Option<String>,
    #[doc = "`--logLevel  <arg>`\n\nOne of the log levels ALL, TRACE, DEBUG, INFO, WARN, ERROR, OFF"]
    #[builder(setter(into, strip_option), default)]
    pub log_level: Option<String>,
    #[doc = "`--mapAxiomatizationFile  <arg>`\n\nSource file with map axiomatisation. If omitted, built-in one is used."]
    #[builder(setter(into, strip_option), default)]
    pub map_axiomatization_file: Option<String>,
    #[doc = "`--maxHeuristicsDepth  <arg>`\n\nMaximal number of nested heuristics applications (default: 3)"]
    #[builder(setter(into, strip_option), default)]
    pub max_heuristics_depth: Option<String>,
    #[doc = "`--multisetAxiomatizationFile  <arg>`\n\nSource file with multiset axiomatisation. If omitted, built-in one is used."]
    #[builder(setter(into, strip_option), default)]
    pub multiset_axiomatization_file: Option<String>,
    #[doc = "`--numberOfErrorsToReport  <arg>`\n\nNumber of errors per member before the verifier stops. If this number is set to 0, all errors are reported."]
    #[builder(setter(into, strip_option), default)]
    pub number_of_errors_to_report: Option<String>,
    #[doc = "`--numberOfParallelVerifiers  <arg>`\n\nNumber of verifiers run in parallel. This number plus one is the number of provers run in parallel (default: 10)"]
    #[builder(setter(into, strip_option), default)]
    pub number_of_parallel_verifiers: Option<String>,
    #[doc = "`--parallelizeBranches`\n\nVerify different branches in parallel."]
    #[builder(setter(strip_option), default)]
    pub parallelize_branches: Option<bool>,
    #[doc = "`--plugin  <arg>`\n\nLoad plugin(s) with given class name(s). Several plugins can be separated by ':'. The fully qualified class name of the plugin should be specified."]
    #[builder(setter(into, strip_option), default)]
    pub plugin: Option<String>,
    #[doc = "`--printMethodCFGs`\n\nPrint a DOT (Graphviz) representation of the CFG of each method to verify to a file '<tempDirectory>/<methodName>.dot'."]
    #[builder(setter(strip_option), default)]
    pub print_method_cf_gs: Option<bool>,
    #[doc = "`--printTranslatedProgram`\n\nPrint the final program that is going to be verified to stdout."]
    #[builder(setter(strip_option), default)]
    pub print_translated_program: Option<bool>,
    #[doc = "`--prover  <arg>`\n\nOne of the provers Z3, cvc5, Z3-API. (default: Z3)."]
    #[builder(setter(into, strip_option), default)]
    pub prover: Option<String>,
    #[doc = "`--proverArgs  <arg>`\n\nCommand-line arguments which should be forwarded to the prover. The expected format is \"<opt> <opt> ... <opt>\", excluding the quotation marks."]
    #[builder(setter(into, strip_option), default)]
    pub prover_args: Option<String>,
    #[doc = "`--proverConfigArgs  <arg>...`\n\nConfiguration options which should be forwarded to the prover. The expected format is \"<key>=<val> <key>=<val> ... <key>=<val>\", excluding the quotation marks. The configuration options given here will override those from Silicon's prover preamble."]
    #[builder(setter(into, strip_option), default)]
    pub prover_config_args: Option<String>,
    #[doc = "`--proverEnableResourceBounds`\n\nUse prover's resource bounds instead of timeouts"]
    #[builder(setter(strip_option), default)]
    pub prover_enable_resource_bounds: Option<bool>,
    #[doc = "`--proverLogFile  <arg>`\n\nLog file containing the interaction with the prover, extension smt2 will be appended. (default: <tempDirectory>/logfile.smt2)"]
    #[builder(setter(into, strip_option), default)]
    pub prover_log_file: Option<String>,
    #[doc = "`--proverRandomizeSeeds`\n\nSet various random seeds of the prover to random values"]
    #[builder(setter(strip_option), default)]
    pub prover_randomize_seeds: Option<bool>,
    #[doc = "`--proverResourcesPerMillisecond  <arg>`\n\nProver resources per milliseconds. Is used to convert timeouts to resource bounds."]
    #[builder(setter(into, strip_option), default)]
    pub prover_resources_per_millisecond: Option<String>,
    #[doc = "`--proverSaturationTimeout  <arg>`\n\nTimeout (in ms) used for the prover's state saturation calls (default: 100). A timeout of 0 disables all saturation checks.Note that for the cvc5 prover, state saturation calls can either be disabled (weights or base timeout of 0) or forced with no timeout (positive weight and base timeout)."]
    #[builder(setter(into, strip_option), default)]
    pub prover_saturation_timeout: Option<String>,
    #[doc = "`--proverSaturationTimeoutWeights  <arg>...`\n\nWeights used to compute the effective timeout for the prover's state saturation calls, which are made at various points during a symbolic execution. The effective timeouts for a particular saturation call is computed by multiplying the corresponding weight with the base timeout for saturation calls. Defaults to the following weights: after program preamble: 1.0 after inhaling contracts: 0.5 after unfold: 0.4 after inhale: 0.2 before repeated prover queries: 0.02 Weights must be non-negative, a weight of 0 disables the corresponding saturation call and a minimal timeout of 10ms is enforced.Note that for the cvc5 prover, state saturation calls can either be disabled (weights or base timeout of 0) or forced with no timeout (positive weight and base timeout)."]
    #[builder(setter(into, strip_option), default)]
    pub prover_saturation_timeout_weights: Option<String>,
    #[doc = "`--pushTimeout  <arg>`\n\nTimeout (in ms) per push operation in the SMT solver. (default: 0, i.e. no timeout). Ignored when using the cvc5 prover."]
    #[builder(setter(into, strip_option), default)]
    pub push_timeout: Option<String>,
    #[doc = "`--qpSplitTimeout  <arg>`\n\nTimeout (in ms) used by QP's split algorithm when 1) checking if a chunk holds no further permissions, and 2) checking if sufficiently many permissions have already been split off."]
    #[builder(setter(into, strip_option), default)]
    pub qp_split_timeout: Option<String>,
    #[doc = "`--recursivePredicateUnfoldings  <arg>`\n\nEvaluate n unfolding expressions in the body of predicates that (transitively) unfold other instances of themselves (default: 1)"]
    #[builder(setter(into, strip_option), default)]
    pub recursive_predicate_unfoldings: Option<String>,
    #[doc = "`--sequenceAxiomatizationFile  <arg>`\n\nSource file with sequence axiomatisation. If omitted, built-in one is used."]
    #[builder(setter(into, strip_option), default)]
    pub sequence_axiomatization_file: Option<String>,
    #[doc = "`--setAxiomatizationFile  <arg>`\n\nSource file with set axiomatisation. If omitted, built-in one is used."]
    #[builder(setter(into, strip_option), default)]
    pub set_axiomatization_file: Option<String>,
    #[doc = "`--stateConsolidationMode  <arg>`\n\nOne of the following modes: 0: Minimal work, many incompletenesses 1: Most work, fewest incompletenesses 2: Similar to 1, but less eager 3: Less eager and less complete than 1 4: Intended for use with"]
    #[builder(setter(into, strip_option), default)]
    pub state_consolidation_mode: Option<String>,
    #[doc = "`--moreCompleteExhale`\n\n"]
    #[builder(setter(strip_option), default)]
    pub more_complete_exhale: Option<bool>,
    #[doc = "`--tempDirectory  <arg>`\n\nPath to which all temporary data will be written (default: ./tmp)"]
    #[builder(setter(into, strip_option), default)]
    pub temp_directory: Option<String>,
    #[doc = "`--timeout  <arg>`\n\nTime out after approx. n seconds. The timeout is for the whole verification, not per method or proof obligation (default: 0, i.e. no timeout)."]
    #[builder(setter(into, strip_option), default)]
    pub timeout: Option<String>,
    #[doc = "`--z3Args  <arg>`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverArgs' instead... Command-line arguments which should be forwarded to Z3. The expected format is \"<opt> <opt> ... <opt>\", excluding the quotation marks."]
    #[builder(setter(into, strip_option), default)]
    pub z3_args: Option<String>,
    #[doc = "`--z3ConfigArgs  <arg>...`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverConfigArgs' instead... Configuration options which should be forwarded to Z3. The expected format is \"<key>=<val> <key>=<val> ... <key>=<val>\", excluding the quotation marks. The configuration options given here will override those from Silicon's Z3 preamble."]
    #[builder(setter(into, strip_option), default)]
    pub z3_config_args: Option<String>,
    #[doc = "`--z3EnableResourceBounds`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverEnableResourceBounds' instead... Use Z3's resource bounds instead of timeouts"]
    #[builder(setter(strip_option), default)]
    pub z3_enable_resource_bounds: Option<bool>,
    #[doc = "`--z3Exe  <arg>`\n\nZ3 executable. The environment variable Z3_EXE can also be used to specify the path of the executable."]
    #[builder(setter(into, strip_option), default)]
    pub z3_exe: Option<String>,
    #[doc = "`--z3LogFile  <arg>`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverLogFile' instead... Log file containing the interaction with the prover, extension smt2 will be appended. (default: <tempDirectory>/logfile.smt2)."]
    #[builder(setter(into, strip_option), default)]
    pub z3_log_file: Option<String>,
    #[doc = "`--z3RandomizeSeeds`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverRandomizeSeeds' instead... Set various Z3 random seeds to random values"]
    #[builder(setter(strip_option), default)]
    pub z3_randomize_seeds: Option<bool>,
    #[doc = "`--z3ResourcesPerMillisecond  <arg>`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverResourcesPerMillisecond' instead... Z3 resources per milliseconds. Is used to convert timeouts to resource bounds."]
    #[builder(setter(into, strip_option), default)]
    pub z3_resources_per_millisecond: Option<String>,
    #[doc = "`--z3SaturationTimeout  <arg>`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverSaturationTimeout' instead... Timeout (in ms) used for Z3 state saturation calls (default: 100). A timeout of 0 disables all saturation checks."]
    #[builder(setter(into, strip_option), default)]
    pub z3_saturation_timeout: Option<String>,
    #[doc = "`--z3SaturationTimeoutWeights  <arg>...`\n\nWarning: This option is deprecated due to standardization in option naming. Please use 'proverSaturationTimeoutWeights' instead... Weights used to compute the effective timeout for Z3 state saturation calls, which are made at various points during a symbolic execution. The effective timeouts for a particular saturation call is computed by multiplying the corresponding weight with the base timeout for saturation calls. Defaults to the following weights: after program preamble: 1.0 after inhaling contracts: 0.5 after unfold: 0.4 after inhale: 0.2 before repeated Z3 queries: 0.02 Weights must be non-negative, a weight of 0 disables the corresponding saturation call and a minimal timeout of 10ms is enforced."]
    #[builder(setter(into, strip_option), default)]
    pub z3_saturation_timeout_weights: Option<String>,
}
impl std::fmt::Display for SiliconOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(true) = &self.alternative_function_verification_order {
            write!(f, "--alternativeFunctionVerificationOrder ")?;
        }
        if let Some(value) = &self.assertion_mode {
            write!(f, "--assertionMode {value} ")?;
        }
        if let Some(value) = &self.assert_timeout {
            write!(f, "--assertTimeout {value} ")?;
        }
        if let Some(true) = &self.assume_injectivity_on_inhale {
            write!(f, "--assumeInjectivityOnInhale ")?;
        }
        if let Some(value) = &self.check_timeout {
            write!(f, "--checkTimeout {value} ")?;
        }
        if let Some(true) = &self.conditionalize_permissions {
            write!(f, "--conditionalizePermissions ")?;
        }
        if let Some(value) = &self.counterexample {
            write!(f, "--counterexample {value} ")?;
        }
        if let Some(value) = &self.cvc5_exe {
            write!(f, "--cvc5Exe {value} ")?;
        }
        if let Some(true) = &self.disable_caches {
            write!(f, "--disableCaches ")?;
        }
        if let Some(true) = &self.disable_catching_exceptions {
            write!(f, "--disableCatchingExceptions ")?;
        }
        if let Some(true) = &self.disable_chunk_order_heuristics {
            write!(f, "--disableChunkOrderHeuristics ")?;
        }
        if let Some(true) = &self.disable_function_unfold_trigger {
            write!(f, "--disableFunctionUnfoldTrigger ")?;
        }
        if let Some(true) = &self.disable_havoc_hack407 {
            write!(f, "--disableHavocHack407 ")?;
        }
        if let Some(true) = &self.disable_isc_triggers {
            write!(f, "--disableISCTriggers ")?;
        }
        if let Some(true) = &self.disable_short_circuiting_evaluations {
            write!(f, "--disableShortCircuitingEvaluations ")?;
        }
        if let Some(true) = &self.disable_subsumption {
            write!(f, "--disableSubsumption ")?;
        }
        if let Some(true) = &self.disable_temp_directory {
            write!(f, "--disableTempDirectory ")?;
        }
        if let Some(true) = &self.disable_value_map_caching {
            write!(f, "--disableValueMapCaching ")?;
        }
        if let Some(true) = &self.enable_branchcondition_reporting {
            write!(f, "--enableBranchconditionReporting ")?;
        }
        if let Some(true) = &self.enable_more_complete_exhale {
            write!(f, "--enableMoreCompleteExhale ")?;
        }
        if let Some(true) = &self.enable_predicate_triggers_on_inhale {
            write!(f, "--enablePredicateTriggersOnInhale ")?;
        }
        if let Some(value) = &self.exclude_methods {
            write!(f, "--excludeMethods {value} ")?;
        }
        if let Some(true) = &self.handle_pure_conjuncts_individually {
            write!(f, "--handlePureConjunctsIndividually ")?;
        }
        if let Some(value) = &self.include_methods {
            write!(f, "--includeMethods {value} ")?;
        }
        if let Some(value) = &self.log_config {
            write!(f, "--logConfig {value} ")?;
        }
        if let Some(value) = &self.log_level {
            write!(f, "--logLevel {value} ")?;
        }
        if let Some(value) = &self.map_axiomatization_file {
            write!(f, "--mapAxiomatizationFile {value} ")?;
        }
        if let Some(value) = &self.max_heuristics_depth {
            write!(f, "--maxHeuristicsDepth {value} ")?;
        }
        if let Some(value) = &self.multiset_axiomatization_file {
            write!(f, "--multisetAxiomatizationFile {value} ")?;
        }
        if let Some(value) = &self.number_of_errors_to_report {
            write!(f, "--numberOfErrorsToReport {value} ")?;
        }
        if let Some(value) = &self.number_of_parallel_verifiers {
            write!(f, "--numberOfParallelVerifiers {value} ")?;
        }
        if let Some(true) = &self.parallelize_branches {
            write!(f, "--parallelizeBranches ")?;
        }
        if let Some(value) = &self.plugin {
            write!(f, "--plugin {value} ")?;
        }
        if let Some(true) = &self.print_method_cf_gs {
            write!(f, "--printMethodCFGs ")?;
        }
        if let Some(true) = &self.print_translated_program {
            write!(f, "--printTranslatedProgram ")?;
        }
        if let Some(value) = &self.prover {
            write!(f, "--prover {value} ")?;
        }
        if let Some(value) = &self.prover_args {
            write!(f, "--proverArgs {value} ")?;
        }
        if let Some(value) = &self.prover_config_args {
            write!(f, "--proverConfigArgs {value} ")?;
        }
        if let Some(true) = &self.prover_enable_resource_bounds {
            write!(f, "--proverEnableResourceBounds ")?;
        }
        if let Some(value) = &self.prover_log_file {
            write!(f, "--proverLogFile {value} ")?;
        }
        if let Some(true) = &self.prover_randomize_seeds {
            write!(f, "--proverRandomizeSeeds ")?;
        }
        if let Some(value) = &self.prover_resources_per_millisecond {
            write!(f, "--proverResourcesPerMillisecond {value} ")?;
        }
        if let Some(value) = &self.prover_saturation_timeout {
            write!(f, "--proverSaturationTimeout {value} ")?;
        }
        if let Some(value) = &self.prover_saturation_timeout_weights {
            write!(f, "--proverSaturationTimeoutWeights {value} ")?;
        }
        if let Some(value) = &self.push_timeout {
            write!(f, "--pushTimeout {value} ")?;
        }
        if let Some(value) = &self.qp_split_timeout {
            write!(f, "--qpSplitTimeout {value} ")?;
        }
        if let Some(value) = &self.recursive_predicate_unfoldings {
            write!(f, "--recursivePredicateUnfoldings {value} ")?;
        }
        if let Some(value) = &self.sequence_axiomatization_file {
            write!(f, "--sequenceAxiomatizationFile {value} ")?;
        }
        if let Some(value) = &self.set_axiomatization_file {
            write!(f, "--setAxiomatizationFile {value} ")?;
        }
        if let Some(value) = &self.state_consolidation_mode {
            write!(f, "--stateConsolidationMode {value} ")?;
        }
        if let Some(true) = &self.more_complete_exhale {
            write!(f, "--moreCompleteExhale ")?;
        }
        if let Some(value) = &self.temp_directory {
            write!(f, "--tempDirectory {value} ")?;
        }
        if let Some(value) = &self.timeout {
            write!(f, "--timeout {value} ")?;
        }
        if let Some(value) = &self.z3_args {
            write!(f, "--z3Args {value} ")?;
        }
        if let Some(value) = &self.z3_config_args {
            write!(f, "--z3ConfigArgs {value} ")?;
        }
        if let Some(true) = &self.z3_enable_resource_bounds {
            write!(f, "--z3EnableResourceBounds ")?;
        }
        if let Some(value) = &self.z3_exe {
            write!(f, "--z3Exe {value} ")?;
        }
        if let Some(value) = &self.z3_log_file {
            write!(f, "--z3LogFile {value} ")?;
        }
        if let Some(true) = &self.z3_randomize_seeds {
            write!(f, "--z3RandomizeSeeds ")?;
        }
        if let Some(value) = &self.z3_resources_per_millisecond {
            write!(f, "--z3ResourcesPerMillisecond {value} ")?;
        }
        if let Some(value) = &self.z3_saturation_timeout {
            write!(f, "--z3SaturationTimeout {value} ")?;
        }
        if let Some(value) = &self.z3_saturation_timeout_weights {
            write!(f, "--z3SaturationTimeoutWeights {value} ")?;
        }
        Ok(())
    }
}
impl SiliconOpts {
    pub fn apply(&self, mut f: impl FnMut(&str)) {
        if let Some(true) = &self.alternative_function_verification_order {
            f("--alternativeFunctionVerificationOrder");
        }
        if let Some(value) = &self.assertion_mode {
            f("--assertionMode");
            f(value);
        }
        if let Some(value) = &self.assert_timeout {
            f("--assertTimeout");
            f(value);
        }
        if let Some(true) = &self.assume_injectivity_on_inhale {
            f("--assumeInjectivityOnInhale");
        }
        if let Some(value) = &self.check_timeout {
            f("--checkTimeout");
            f(value);
        }
        if let Some(true) = &self.conditionalize_permissions {
            f("--conditionalizePermissions");
        }
        if let Some(value) = &self.counterexample {
            f("--counterexample");
            f(value);
        }
        if let Some(value) = &self.cvc5_exe {
            f("--cvc5Exe");
            f(value);
        }
        if let Some(true) = &self.disable_caches {
            f("--disableCaches");
        }
        if let Some(true) = &self.disable_catching_exceptions {
            f("--disableCatchingExceptions");
        }
        if let Some(true) = &self.disable_chunk_order_heuristics {
            f("--disableChunkOrderHeuristics");
        }
        if let Some(true) = &self.disable_function_unfold_trigger {
            f("--disableFunctionUnfoldTrigger");
        }
        if let Some(true) = &self.disable_havoc_hack407 {
            f("--disableHavocHack407");
        }
        if let Some(true) = &self.disable_isc_triggers {
            f("--disableISCTriggers");
        }
        if let Some(true) = &self.disable_short_circuiting_evaluations {
            f("--disableShortCircuitingEvaluations");
        }
        if let Some(true) = &self.disable_subsumption {
            f("--disableSubsumption");
        }
        if let Some(true) = &self.disable_temp_directory {
            f("--disableTempDirectory");
        }
        if let Some(true) = &self.disable_value_map_caching {
            f("--disableValueMapCaching");
        }
        if let Some(true) = &self.enable_branchcondition_reporting {
            f("--enableBranchconditionReporting");
        }
        if let Some(true) = &self.enable_more_complete_exhale {
            f("--enableMoreCompleteExhale");
        }
        if let Some(true) = &self.enable_predicate_triggers_on_inhale {
            f("--enablePredicateTriggersOnInhale");
        }
        if let Some(value) = &self.exclude_methods {
            f("--excludeMethods");
            f(value);
        }
        if let Some(true) = &self.handle_pure_conjuncts_individually {
            f("--handlePureConjunctsIndividually");
        }
        if let Some(value) = &self.include_methods {
            f("--includeMethods");
            f(value);
        }
        if let Some(value) = &self.log_config {
            f("--logConfig");
            f(value);
        }
        if let Some(value) = &self.log_level {
            f("--logLevel");
            f(value);
        }
        if let Some(value) = &self.map_axiomatization_file {
            f("--mapAxiomatizationFile");
            f(value);
        }
        if let Some(value) = &self.max_heuristics_depth {
            f("--maxHeuristicsDepth");
            f(value);
        }
        if let Some(value) = &self.multiset_axiomatization_file {
            f("--multisetAxiomatizationFile");
            f(value);
        }
        if let Some(value) = &self.number_of_errors_to_report {
            f("--numberOfErrorsToReport");
            f(value);
        }
        if let Some(value) = &self.number_of_parallel_verifiers {
            f("--numberOfParallelVerifiers");
            f(value);
        }
        if let Some(true) = &self.parallelize_branches {
            f("--parallelizeBranches");
        }
        if let Some(value) = &self.plugin {
            f("--plugin");
            f(value);
        }
        if let Some(true) = &self.print_method_cf_gs {
            f("--printMethodCFGs");
        }
        if let Some(true) = &self.print_translated_program {
            f("--printTranslatedProgram");
        }
        if let Some(value) = &self.prover {
            f("--prover");
            f(value);
        }
        if let Some(value) = &self.prover_args {
            f("--proverArgs");
            f(value);
        }
        if let Some(value) = &self.prover_config_args {
            f("--proverConfigArgs");
            f(value);
        }
        if let Some(true) = &self.prover_enable_resource_bounds {
            f("--proverEnableResourceBounds");
        }
        if let Some(value) = &self.prover_log_file {
            f("--proverLogFile");
            f(value);
        }
        if let Some(true) = &self.prover_randomize_seeds {
            f("--proverRandomizeSeeds");
        }
        if let Some(value) = &self.prover_resources_per_millisecond {
            f("--proverResourcesPerMillisecond");
            f(value);
        }
        if let Some(value) = &self.prover_saturation_timeout {
            f("--proverSaturationTimeout");
            f(value);
        }
        if let Some(value) = &self.prover_saturation_timeout_weights {
            f("--proverSaturationTimeoutWeights");
            f(value);
        }
        if let Some(value) = &self.push_timeout {
            f("--pushTimeout");
            f(value);
        }
        if let Some(value) = &self.qp_split_timeout {
            f("--qpSplitTimeout");
            f(value);
        }
        if let Some(value) = &self.recursive_predicate_unfoldings {
            f("--recursivePredicateUnfoldings");
            f(value);
        }
        if let Some(value) = &self.sequence_axiomatization_file {
            f("--sequenceAxiomatizationFile");
            f(value);
        }
        if let Some(value) = &self.set_axiomatization_file {
            f("--setAxiomatizationFile");
            f(value);
        }
        if let Some(value) = &self.state_consolidation_mode {
            f("--stateConsolidationMode");
            f(value);
        }
        if let Some(true) = &self.more_complete_exhale {
            f("--moreCompleteExhale");
        }
        if let Some(value) = &self.temp_directory {
            f("--tempDirectory");
            f(value);
        }
        if let Some(value) = &self.timeout {
            f("--timeout");
            f(value);
        }
        if let Some(value) = &self.z3_args {
            f("--z3Args");
            f(value);
        }
        if let Some(value) = &self.z3_config_args {
            f("--z3ConfigArgs");
            f(value);
        }
        if let Some(true) = &self.z3_enable_resource_bounds {
            f("--z3EnableResourceBounds");
        }
        if let Some(value) = &self.z3_exe {
            f("--z3Exe");
            f(value);
        }
        if let Some(value) = &self.z3_log_file {
            f("--z3LogFile");
            f(value);
        }
        if let Some(true) = &self.z3_randomize_seeds {
            f("--z3RandomizeSeeds");
        }
        if let Some(value) = &self.z3_resources_per_millisecond {
            f("--z3ResourcesPerMillisecond");
            f(value);
        }
        if let Some(value) = &self.z3_saturation_timeout {
            f("--z3SaturationTimeout");
            f(value);
        }
        if let Some(value) = &self.z3_saturation_timeout_weights {
            f("--z3SaturationTimeoutWeights");
            f(value);
        }
    }
}
