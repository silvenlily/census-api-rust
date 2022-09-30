use std::{sync::Arc, time::SystemTime};

use num_traits::ToPrimitive;
use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) struct CensusValue<T: Clone> {
    pub last_updated: Option<SystemTime>,
    pub value: Option<T>,
}

impl<T: Clone> CensusValue<T> {
    pub fn new_empty<V: Clone>() -> CensusValue<V> {
        CensusValue {
            last_updated: None,
            value: None,
        }
    }

    pub fn new_value<V: Clone>(value: V) -> CensusValue<V> {
        CensusValue {
            value: Some(value),
            last_updated: Some(SystemTime::now()),
        }
    }
}

impl CensusValue<String> {
    pub fn from_json(json: &Value) -> CensusValue<String> {
        let mut val: CensusValue<String> = CensusValue {
            value: None,
            last_updated: None,
        };

        val.update(json);

        return val;
    }

    pub fn update(&mut self, json: &Value) -> bool {
        if json.is_string() {
            self.value = Some(json.as_str().unwrap().to_string());
            self.last_updated = Some(SystemTime::now());

            return true;
        }

        return false;
    }
}

impl CensusValue<u8> {
    pub fn from_json(json: &Value) -> CensusValue<u8> {
        let mut val: CensusValue<u8> = CensusValue {
            value: None,
            last_updated: None,
        };

        val.update(json);

        return val;
    }

    pub fn update(&mut self, json: &Value) -> bool {
        if json.is_number() {
            let try_val = json.as_u64();

            match try_val {
                None => {}
                Some(v_64) => match v_64.to_u8() {
                    None => {}
                    Some(v) => {
                        self.value = Some(v);
                        self.last_updated = Some(SystemTime::now());
                        return true;
                    }
                },
            }
        }

        if json.is_string() {
            let try_str = json.as_str().unwrap();

            let try_val = try_str.parse::<u8>();
            match try_val {
                Err(_) => {}
                Ok(v) => {
                    self.value = Some(v);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
            }
        }

        return false;
    }

    pub fn percent_from_float_json(json: &Value) -> CensusValue<u8> {
        let mut val: CensusValue<u8> = CensusValue {
            value: None,
            last_updated: None,
        };
        val.update(json);
        return val;
    }

    pub fn percent_update_float_json(&mut self, json: &Value) -> bool {
        if json.is_number() {
            let try_val = json.as_f64();

            match try_val {
                Some(vf_f64) => {
                    let v_f64 = (vf_f64 * 100_f64).floor();

                    match v_f64.to_u8() {
                        Some(v) => {
                            self.value = Some(v);
                            self.last_updated = Some(SystemTime::now());
                            return true;
                        }
                        None => {
                            return false;
                        }
                    };
                }
                None => {
                    return false;
                }
            };
        };

        if json.is_string() {
            let try_str = json.as_str().unwrap();

            let try_val = try_str.parse::<f64>();
            match try_val {
                Err(_) => {}
                Ok(vf_f64) => {
                    let v_f64 = (vf_f64 * 100_f64).floor();

                    match v_f64.to_u8() {
                        Some(v) => {
                            self.value = Some(v);
                            self.last_updated = Some(SystemTime::now());
                            return true;
                        }
                        None => {
                            return false;
                        }
                    };
                }
            };
        };

        return false;
    }
}

impl CensusValue<u64> {
    pub fn from_json(json: &Value) -> Self {
        let mut val: CensusValue<u64> = CensusValue {
            value: None,
            last_updated: None,
        };

        val.update(json);

        return val;
    }

    pub fn update(&mut self, json: &Value) -> bool {
        if json.is_number() {
            let try_val = json.as_u64();

            match try_val {
                Some(v) => {
                    self.value = Some(v);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
                None => {}
            }
        }

        if json.is_string() {
            let try_str = json.as_str().unwrap();

            let try_val = try_str.parse::<u64>();
            match try_val {
                Err(_) => {}
                Ok(v) => {
                    self.value = Some(v);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
            }
        }

        return false;
    }
}

impl CensusValue<bool> {
    pub fn from_json(json: &Value) -> Self {
        let mut val: CensusValue<bool> = CensusValue {
            value: None,
            last_updated: None,
        };

        val.update(json);

        return val;
    }
    pub fn update(&mut self, json: &Value) -> bool {
        if json.is_string() {
            let str = json.as_str().unwrap();

            let v: Option<bool>;

            match str.to_lowercase().as_str() {
                "true" => {
                    self.value = Some(true);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
                "false" => {
                    self.value = Some(false);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
                "1" => {
                    self.value = Some(true);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
                "0" => {
                    self.value = Some(false);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }

        if json.is_boolean() {
            let try_bool = json.as_bool();

            match try_bool {
                None => {
                    return false;
                }
                Some(v) => {
                    self.value = Some(v);
                    self.last_updated = Some(SystemTime::now());
                    return true;
                }
            }
        }

        if json.is_number() {
            let try_num = json.as_u64();

            match try_num {
                None => {}
                Some(num) => match num {
                    0 => {
                        self.value = Some(false);
                        self.last_updated = Some(SystemTime::now());
                        return true;
                    }
                    1 => {
                        self.value = Some(true);
                        self.last_updated = Some(SystemTime::now());
                        return true;
                    }
                    _ => {
                        return false;
                    }
                },
            }
        }

        return false;
    }
}
