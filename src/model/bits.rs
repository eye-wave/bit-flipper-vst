use nih_plug::prelude::*;

#[derive(Params, Debug)]
pub struct BitParams {
    #[id = "mask_bit_1"]
    pub mask_bit_1: BoolParam,
    #[id = "mask_bit_2"]
    pub mask_bit_2: BoolParam,
    #[id = "mask_bit_3"]
    pub mask_bit_3: BoolParam,
    #[id = "mask_bit_4"]
    pub mask_bit_4: BoolParam,
    #[id = "mask_bit_5"]
    pub mask_bit_5: BoolParam,
    #[id = "mask_bit_6"]
    pub mask_bit_6: BoolParam,
    #[id = "mask_bit_7"]
    pub mask_bit_7: BoolParam,
    #[id = "mask_bit_8"]
    pub mask_bit_8: BoolParam,
    #[id = "mask_bit_9"]
    pub mask_bit_9: BoolParam,

    #[id = "mask_bit_10"]
    pub mask_bit_10: BoolParam,
    #[id = "mask_bit_11"]
    pub mask_bit_11: BoolParam,
    #[id = "mask_bit_12"]
    pub mask_bit_12: BoolParam,
    #[id = "mask_bit_13"]
    pub mask_bit_13: BoolParam,
    #[id = "mask_bit_14"]
    pub mask_bit_14: BoolParam,
    #[id = "mask_bit_15"]
    pub mask_bit_15: BoolParam,
    #[id = "mask_bit_16"]
    pub mask_bit_16: BoolParam,
    #[id = "mask_bit_17"]
    pub mask_bit_17: BoolParam,
    #[id = "mask_bit_18"]
    pub mask_bit_18: BoolParam,
    #[id = "mask_bit_19"]
    pub mask_bit_19: BoolParam,

    #[id = "mask_bit_20"]
    pub mask_bit_20: BoolParam,
    #[id = "mask_bit_21"]
    pub mask_bit_21: BoolParam,
    #[id = "mask_bit_22"]
    pub mask_bit_22: BoolParam,
    #[id = "mask_bit_23"]
    pub mask_bit_23: BoolParam,
    #[id = "mask_bit_24"]
    pub mask_bit_24: BoolParam,
    #[id = "mask_bit_25"]
    pub mask_bit_25: BoolParam,
    #[id = "mask_bit_26"]
    pub mask_bit_26: BoolParam,
    #[id = "mask_bit_27"]
    pub mask_bit_27: BoolParam,
    #[id = "mask_bit_28"]
    pub mask_bit_28: BoolParam,
    #[id = "mask_bit_29"]
    pub mask_bit_29: BoolParam,

    #[id = "mask_bit_30"]
    pub mask_bit_30: BoolParam,
    #[id = "mask_bit_31"]
    pub mask_bit_31: BoolParam,
    #[id = "mask_bit_32"]
    pub mask_bit_32: BoolParam,
}

macro_rules! add_bit {
    ($result:ident, $self:ident, $field:ident, $bit:expr) => {
        $result |= ($self.$field.value() as u32) << $bit;
    };
}

impl BitParams {
    pub fn get_bit_param(&self, id: u8) -> Option<&BoolParam> {
        match id {
            1 => Some(&self.mask_bit_1),
            2 => Some(&self.mask_bit_2),
            3 => Some(&self.mask_bit_3),
            4 => Some(&self.mask_bit_4),
            5 => Some(&self.mask_bit_5),
            6 => Some(&self.mask_bit_6),
            7 => Some(&self.mask_bit_7),
            8 => Some(&self.mask_bit_8),
            9 => Some(&self.mask_bit_9),
            10 => Some(&self.mask_bit_10),
            11 => Some(&self.mask_bit_11),
            12 => Some(&self.mask_bit_12),
            13 => Some(&self.mask_bit_13),
            14 => Some(&self.mask_bit_14),
            15 => Some(&self.mask_bit_15),
            16 => Some(&self.mask_bit_16),
            17 => Some(&self.mask_bit_17),
            18 => Some(&self.mask_bit_18),
            19 => Some(&self.mask_bit_19),
            20 => Some(&self.mask_bit_20),
            21 => Some(&self.mask_bit_21),
            22 => Some(&self.mask_bit_22),
            23 => Some(&self.mask_bit_23),
            24 => Some(&self.mask_bit_24),
            25 => Some(&self.mask_bit_25),
            26 => Some(&self.mask_bit_26),
            27 => Some(&self.mask_bit_27),
            28 => Some(&self.mask_bit_28),
            29 => Some(&self.mask_bit_29),
            30 => Some(&self.mask_bit_30),
            31 => Some(&self.mask_bit_31),
            32 => Some(&self.mask_bit_32),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut result = 0u32;

        add_bit!(result, self, mask_bit_1, 0);
        add_bit!(result, self, mask_bit_2, 1);
        add_bit!(result, self, mask_bit_3, 2);
        add_bit!(result, self, mask_bit_4, 3);
        add_bit!(result, self, mask_bit_5, 4);
        add_bit!(result, self, mask_bit_6, 5);
        add_bit!(result, self, mask_bit_7, 6);
        add_bit!(result, self, mask_bit_8, 7);
        add_bit!(result, self, mask_bit_9, 8);
        add_bit!(result, self, mask_bit_10, 9);
        add_bit!(result, self, mask_bit_11, 10);
        add_bit!(result, self, mask_bit_12, 11);
        add_bit!(result, self, mask_bit_13, 12);
        add_bit!(result, self, mask_bit_14, 13);
        add_bit!(result, self, mask_bit_15, 14);
        add_bit!(result, self, mask_bit_16, 15);
        add_bit!(result, self, mask_bit_17, 16);
        add_bit!(result, self, mask_bit_18, 17);
        add_bit!(result, self, mask_bit_19, 18);
        add_bit!(result, self, mask_bit_20, 19);
        add_bit!(result, self, mask_bit_21, 20);
        add_bit!(result, self, mask_bit_22, 21);
        add_bit!(result, self, mask_bit_23, 22);
        add_bit!(result, self, mask_bit_24, 23);
        add_bit!(result, self, mask_bit_25, 24);
        add_bit!(result, self, mask_bit_26, 25);
        add_bit!(result, self, mask_bit_27, 26);
        add_bit!(result, self, mask_bit_28, 27);
        add_bit!(result, self, mask_bit_29, 28);
        add_bit!(result, self, mask_bit_30, 29);
        add_bit!(result, self, mask_bit_31, 30);
        add_bit!(result, self, mask_bit_32, 31);

        result
    }
}

impl Default for BitParams {
    fn default() -> Self {
        Self {
            mask_bit_1: BoolParam::new("mask_bit_1", false),
            mask_bit_2: BoolParam::new("mask_bit_2", false),
            mask_bit_3: BoolParam::new("mask_bit_3", false),
            mask_bit_4: BoolParam::new("mask_bit_4", false),
            mask_bit_5: BoolParam::new("mask_bit_5", false),
            mask_bit_6: BoolParam::new("mask_bit_6", false),
            mask_bit_7: BoolParam::new("mask_bit_7", false),
            mask_bit_8: BoolParam::new("mask_bit_8", false),
            mask_bit_9: BoolParam::new("mask_bit_9", false),

            mask_bit_10: BoolParam::new("mask_bit_10", false),
            mask_bit_11: BoolParam::new("mask_bit_11", false),
            mask_bit_12: BoolParam::new("mask_bit_12", false),
            mask_bit_13: BoolParam::new("mask_bit_13", false),
            mask_bit_14: BoolParam::new("mask_bit_14", false),
            mask_bit_15: BoolParam::new("mask_bit_15", false),
            mask_bit_16: BoolParam::new("mask_bit_16", false),
            mask_bit_17: BoolParam::new("mask_bit_17", false),
            mask_bit_18: BoolParam::new("mask_bit_18", false),
            mask_bit_19: BoolParam::new("mask_bit_19", false),

            mask_bit_20: BoolParam::new("mask_bit_20", false),
            mask_bit_21: BoolParam::new("mask_bit_21", false),
            mask_bit_22: BoolParam::new("mask_bit_22", false),
            mask_bit_23: BoolParam::new("mask_bit_23", false),
            mask_bit_24: BoolParam::new("mask_bit_24", false),
            mask_bit_25: BoolParam::new("mask_bit_25", false),
            mask_bit_26: BoolParam::new("mask_bit_26", false),
            mask_bit_27: BoolParam::new("mask_bit_27", false),
            mask_bit_28: BoolParam::new("mask_bit_28", false),
            mask_bit_29: BoolParam::new("mask_bit_29", false),

            mask_bit_30: BoolParam::new("mask_bit_30", false),
            mask_bit_31: BoolParam::new("mask_bit_31", false),
            mask_bit_32: BoolParam::new("mask_bit_32", false),
        }
    }
}
