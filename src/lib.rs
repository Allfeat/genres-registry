// This file is part of Allfeat.

// Copyright (C) Allfeat (FR) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[cfg(feature = "substrate")]
use sp_runtime::RuntimeDebug;

declare_music_genre! {
    Classical(ClassicalSubtype {
        Baroque,
        Romantic,
        Modern,
        Contemporary,
        ChamberMusic,
        Symphony,
    }),
    Jazz(JazzSubtype {
        Bebop,
        Fusion,
        Smooth,
        Swing,
        Cool,
        FreeJazz,
    }),
    Rock(RockSubtype {
        ClassicRock,
        HardRock,
        AlternativeRock,
        ProgressiveRock,
        PunkRock,
        Psychedelic,
    }),
    Electronic(ElectronicSubtype {
        Techno,
        House,
        Trance,
        Ambient,
        DrumNBass,
        IDM,
    }),
    Rap(RapSubtype {
        EastCoast,
        WestCoast,
        Trap,
        Gangsta,
        Conscious,
        AlternativeRap,
    }),
    Pop(PopSubtype {
        SynthPop,
        DancePop,
        TeenPop,
        BritPop,
        ArtPop,
    }),
    Country(CountrySubtype {
        Bluegrass,
        HonkyTonk,
        OutlawCountry,
        CountryRock,
        Contemporary,
    }),
    Blues(BluesSubtype {
        DeltaBlues,
        ChicagoBlues,
        ElectricBlues,
        RhythmAndBlues,
    }),
    Soul(SoulSubtype {
        NeoSoul,
        BlueEyedSoul,
        NorthernSoul,
        Motown,
    }),
    Reggae(ReggaeSubtype {
        Dancehall,
        Dub,
        RootsReggae,
    }),
    Folk(FolkSubtype {
        TraditionalFolk,
        ContemporaryFolk,
        FolkRock,
    }),
    RnB(RnBSubtype {
        ContemporaryRnB,
        NewJackSwing,
        Funk,
    }),
    Metal(MetalSubtype {
        HeavyMetal,
        DeathMetal,
        BlackMetal,
        PowerMetal,
        ThrashMetal,
        NuMetal,
        DoomMetal,
        GothicMetal,
    }),
    Punk(PunkSubtype {
        HardcorePunk,
        PostPunk,
        AnarchoPunk,
        PopPunk,
    }),
    Indie(IndieSubtype {
        IndieRock,
        IndiePop,
        IndieElectro,
        LoFi,
    }),
    EDM(EDMSubtype {
        ProgressiveHouse,
        BigRoom,
        Dubstep,
        TropicalHouse,
        FutureBass,
    }),
    Latin(LatinSubtype {
        Salsa,
        Bachata,
        Reggaeton,
        Mariachi,
        Tango,
    }),
    World(WorldSubtype {
        African,
        Caribbean,
        MiddleEastern,
        Asian,
        Celtic,
    }),
}
