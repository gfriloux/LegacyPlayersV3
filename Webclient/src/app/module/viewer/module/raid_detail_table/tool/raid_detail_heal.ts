import {InstanceDataFilter} from "../../../tool/instance_data_filter";
import {HitType} from "../../../domain_value/hit_type";
import {DetailRow} from "../domain_value/detail_row";
import {commit_heal_detail} from "../stdlib/heal_detail";
import {HealMode} from "../../../domain_value/heal_mode";
import {Heal} from "../../../domain_value/event";

export class RaidDetailHeal {

    constructor(
        private data_filter: InstanceDataFilter
    ) {
    }

    async calculate(heal_mode: HealMode, inverse: boolean): Promise<Array<[number, Array<[HitType, DetailRow]>]>> {
        if (inverse)
            return commit_heal_detail(heal_mode, this.data_filter.get_heal(true) as Array<Heal>);
        return commit_heal_detail(heal_mode, this.data_filter.get_heal(false) as Array<Heal>);
    }
}
