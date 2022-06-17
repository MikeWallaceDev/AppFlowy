import 'package:flutter/material.dart';

import 'package:app_flowy/workspace/application/grid/field/type_option/checklist_select_type_option.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/header/field_editor_pannel.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/header/type_option/select_option.dart';
import 'select_option.dart';


class ChecklistSelectTypeOptionBuilder extends TypeOptionBuilder {
  final ChecklistSelectTypeOptionWidget _widget;

  ChecklistSelectTypeOptionBuilder(
    ChecklistSelectTypeOptionContext typeOptionContext,
    TypeOptionOverlayDelegate overlayDelegate,
  ) : _widget = ChecklistSelectTypeOptionWidget(
          typeOptionContext: typeOptionContext,
          overlayDelegate: overlayDelegate,
        );

  @override
  Widget? get customWidget => _widget;
}

class ChecklistSelectTypeOptionWidget extends TypeOptionWidget {
  final ChecklistSelectTypeOptionContext typeOptionContext;
  final TypeOptionOverlayDelegate overlayDelegate;

  const ChecklistSelectTypeOptionWidget({
    required this.typeOptionContext,
    required this.overlayDelegate,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SelectOptionTypeOptionWidget(
      options: typeOptionContext.typeOption.options,
      beginEdit: () => overlayDelegate.hideOverlay(context),
      overlayDelegate: overlayDelegate,
      typeOptionAction: typeOptionContext,
      // key: ValueKey(state.typeOption.hashCode),
    );
  }
}