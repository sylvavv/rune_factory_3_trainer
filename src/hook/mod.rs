mod asm;

pub(crate) static mut COIN_ADDR: *mut u32 = ::core::ptr::null_mut();

pub(crate) mod fishing {
    // target 5 + 3
    // size 37
    pub(crate) const PAT: &str = "0F B7 48 18 66 83 F9 03";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod auto_press {
    // target 5 + 1
    // size 25
    pub(crate) const PAT: &str = "66 F7 D2 66 23 D0";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut AUTO_PRESS_MARK: i64 = 0;
    pub(crate) static mut AUTO_PRESS_MARK_POINTER: *const i64 =
        unsafe { std::ptr::addr_of!(crate::hook::auto_press::AUTO_PRESS_MARK) } as *const i64;
}

pub(crate) mod walk_through_walls {
    // target 5 + 1
    pub(crate) const PAT: &str = "48 8B F2 48 85 C9";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod friendship_mul {
    // target 5 + 1
    pub(crate) const PAT: &str = "44 8B CA 4D 85 DB";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod instant_crop_growth {
    // target 5 + 1
    pub(crate) const PAT: &str = "8B 10 D1 EA 83 E2 7F 74";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod skill_exp_mul {
    // target 5 + 1
    pub(crate) const PAT: &str = "4C 63 C2 0F B7 CE";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;
}

pub(crate) mod farm {
    // target 5 + 3
    pub(crate) const PAT: &str = "48 83 C3 08 66 41 3B FF";
    pub(crate) static mut HOOK: crate::hook::Hook = unsafe { ::core::mem::zeroed() };
    pub(crate) static mut TOGGLE: bool = false;

    pub(crate) mod soil_quality {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
        pub(crate) static mut SOIL_QUALITY_MARK_POINTER: *const i64 =
            unsafe { std::ptr::addr_of!(crate::hook::farm::soil_quality::MARK) } as *const i64;
    }

    pub(crate) mod watering_plots {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
        pub(crate) static mut WATERING_PLOTS_MARK_POINTER: *const i64 =
            unsafe { std::ptr::addr_of!(crate::hook::farm::watering_plots::MARK) } as *const i64;
    }

    pub(crate) mod tilth_plots {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut TOGGLE: bool = false;
        pub(crate) static mut TILTT_PLOTS_MARK_POINTER: *const i64 =
            unsafe { std::ptr::addr_of!(crate::hook::farm::tilth_plots::MARK) } as *const i64;
    }

    pub(crate) mod plant_plots {
        pub(crate) static mut MARK: i64 = 0;
        pub(crate) static mut PLANT_PLOTS_MARK_POINTER: *const i64 =
            unsafe { std::ptr::addr_of!(crate::hook::farm::plant_plots::MARK) } as *const i64;
        pub(crate) static mut TOGGLE: bool = false;
        pub(crate) static mut CROP_PROP: crate::hook::CropProp = crate::hook::CropProp {
            ty: 0,
            growth_stage_and_lv: 0x10,
        };
        pub(crate) static mut CROP_PROP_POINTER: *const crate::hook::CropProp =
            unsafe { std::ptr::addr_of!(crate::hook::farm::plant_plots::CROP_PROP) }
                as *const crate::hook::CropProp;
    }
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub(crate) struct CropProp {
    ty: u8,
    growth_stage_and_lv: u8,
}

impl CropProp {
    pub(crate) fn set_crop_type(&mut self, ct: CropType) {
        if ct as u8 == 0 {
            self.ty = 0;
        }
        self.ty = (ct as u8) << 1;
    }

    pub(crate) unsafe fn set_crop_growth_stage(&mut self, stage: CropGrowthStage) {
        self.growth_stage_and_lv &= 0b0000_1111;
        self.growth_stage_and_lv |= (stage as u8) << 4;
    }

    pub(crate) fn set_crop_level(&mut self, level: CropLevel) {
        self.growth_stage_and_lv &= 0b0111_0000;
        self.growth_stage_and_lv |= level as u8;
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropType {
    无 = 0,

    石头 = 1, // 可捡
    岩石 = 2, // 可砸
    树枝 = 3, // 可捡
    树桩 = 4, // 可劈
    木材 = 5, // 可砸，什么都不会出
    毒沼 = 6, // 可砸，什么都不会出

    // 矿石 = 7, // 锤子砸会闪退
    药草 = 8,    // 可捡
    解毒草 = 9,  // 可捡
    黑草 = 10,   // 可捡
    枯草 = 11,   // 可捡
    黄草 = 14,   // 可捡
    苦橙草 = 15, // 可捡

    // 种子 = 16, // 不可捡。名字就叫 “种子
    杂草 = 17,   // 可捡
    季节岩 = 18, // 可砸
    花卉 = 19,   // 可摧毁

    水晶 = 20, // 可砸，出的不知道是不是buff

    // 苹果 = 21, //  可砸，什么都不会出
    // 苹果 = 22    同上
    // 苹果 = 23    同上
    草莓 = 24,     // Strawberry
    卷心菜 = 25,   // Cabbage
    樱芜菁 = 26,   // Pink Turnip
    洋葱 = 27,     // Onion
    托伊药草 = 28, // Toyherb
    月落草 = 29,   // Moondrop Flower
    樱草 = 30,     // Cherry Grass
    灯草 = 31,     // Lamp Grass
    青水晶 = 33,   // Blue Crystal Flower
    金卷心菜 = 34, // Golden King Cabbage
    少女蜜瓜 = 35, // Pink Melon

    竹笋 = 36, // 可割

    南瓜 = 37,     // Pumpkin
    黄瓜 = 38,     // Cucumber
    玉米 = 39,     // Corn
    番茄 = 40,     // Tomato
    茄子 = 41,     // Eggplant
    菠萝 = 42,     // Pineapple
    粉红猫 = 43,   // Pink Cat
    铁千轮 = 44,   // Ironleaf
    四叶草 = 45,   // 4-Leaf Clover
    原之焰火 = 46, // Fireflower
    绿水晶 = 47,   // Green Crystal Flower
    金南瓜 = 48,   // Golden Pumpkin

    蓝草 = 49, // 可捡
    绿草 = 50, // 可捡
    紫草 = 51, // 可捡
    靛草 = 52, // 可捡

    红叶花 = 59,     // Autumn Grass
    剧毒蒲公英 = 60, // Pom-Pom Grass
    红水晶 = 61,     // Red Crystal Flower
    金马铃薯 = 62,   // Golden Potato
    芜菁 = 63,       // Turnip
    白萝卜 = 64,     // Radish
    葱 = 65,         // Leek
    白菜 = 66,       // Napa Cabbage
    树形草 = 67,     // Noel Grass
    白水晶 = 68,     // White Crystal Flower
    金芜青 = 69,     // Golden Turnip
    火热果实 = 70,   // Hot-Hot Fruit

    白草 = 71, // 可捡
               //无效 = 72  从72开始的编号都是无效的东西
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]

pub(crate) enum CropLevel {
    LV1 = 0,
    LV2 = 1,
    LV3 = 2,
    LV4 = 3,
    LV5 = 4,
    LV6 = 5,
    LV7 = 6,
    LV8 = 7,
    LV9 = 8,
    LV10 = 9,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropGrowthStage {
    // 无 = 0,
    一阶段 = 0x1,
    二阶段 = 0x2,
    三阶段 = 0x3,
    四阶段 = 0x4,
    五阶段 = 0x5,
}

#[derive(Clone, Copy)]
pub(crate) struct Hook {
    pub(crate) target_addr: *mut std::ffi::c_void,
    pub(crate) target_back_addr: *mut std::ffi::c_void,
    pub(crate) detour_fn_addr: *mut std::ffi::c_void,
    pub(crate) is_enable: bool,
}

impl Hook {
    pub(crate) unsafe fn new() -> Self {
        ::core::mem::zeroed()
    }

    pub(crate) unsafe fn build_target(
        &mut self,
        mod_addr: *mut ::core::ffi::c_void,
        mod_data: &[u8],
        pat: &str,
        occupied: usize,
    ) -> &mut Self {
        let pat_offset = vcheat::pat_find(pat, mod_data).unwrap();

        self.target_addr = mod_addr.add(pat_offset);
        self.target_back_addr = self.target_addr.add(occupied);

        self
    }

    pub(crate) unsafe fn build_detour(
        &mut self,
        detour_fn_addr: *mut ::core::ffi::c_void,
        scan_nop_max_size: usize,
    ) -> &mut Self {
        self.detour_fn_addr = detour_fn_addr;

        let mut detour_fn_end_offset = 0;

        let mut mask = 0;

        for i in 0..scan_nop_max_size {
            if detour_fn_addr.add(i).cast::<u8>().read() == 0x90 {
                mask += 1;
                if mask == 4 {
                    detour_fn_end_offset = i - 4;
                    break;
                }
            }
        }

        let mut jmp_target_addr_shell_code = Vec::new();

        jmp_target_addr_shell_code.push(0xFF);
        jmp_target_addr_shell_code.push(0x25);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);
        jmp_target_addr_shell_code.push(0x0);

        jmp_target_addr_shell_code
            .extend_from_slice((self.target_back_addr as isize).to_le_bytes().as_ref());

        vcheat::write_mem(
            vcheat::internal::get_proc_handle(),
            detour_fn_addr.add(detour_fn_end_offset + 1),
            &jmp_target_addr_shell_code,
        )
        .unwrap();

        self
    }

    pub(crate) unsafe fn create_hook(&self) -> &Self {
        minhook_raw::create_hook(
            self.target_addr,
            self.detour_fn_addr,
            ::core::ptr::null_mut(),
        );

        self
    }

    pub(crate) fn enable(&mut self) {
        minhook_raw::enable_hook(self.target_addr);
        self.is_enable = true;
    }

    pub(crate) fn disable(&mut self) {
        minhook_raw::disable_hook(self.target_addr);

        self.is_enable = false;
    }
}

pub(crate) unsafe fn hooks(mod_addr: *mut ::core::ffi::c_void, mod_data: &[u8]) {
    crate::hook::auto_press::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::auto_press::PAT, 6)
        .build_detour(asm::auto_press as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::fishing::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::fishing::PAT, 8)
        .build_detour(asm::fishing as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::walk_through_walls::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::walk_through_walls::PAT, 6)
        .build_detour(asm::walk_through_walls as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::friendship_mul::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::friendship_mul::PAT, 6)
        .build_detour(asm::friendship_mul as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::instant_crop_growth::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::instant_crop_growth::PAT, 7)
        .build_detour(asm::crop_instant_growth as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::skill_exp_mul::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::skill_exp_mul::PAT, 6)
        .build_detour(asm::skill_exp_mul as *mut ::core::ffi::c_void, 64)
        .create_hook()
        .to_owned();

    crate::hook::farm::HOOK = Hook::new()
        .build_target(mod_addr, mod_data, crate::hook::farm::PAT, 8)
        .build_detour(asm::farm as *mut ::core::ffi::c_void, 0xFF)
        .create_hook()
        .to_owned();

    crate::hook::COIN_ADDR = (crate::SANDLL_ADDR + 0x2AD192C) as *mut u32;
}